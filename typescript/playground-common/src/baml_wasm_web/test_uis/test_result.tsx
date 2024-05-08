import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { VSCodeButton, VSCodeProgressRing, VSCodeTextField } from '@vscode/webview-ui-toolkit/react'
import { atom, useAtom, useAtomValue, useSetAtom } from 'jotai'
import { PropsWithChildren, useMemo, useState } from 'react'
import {
  TestState,
  type TestStatusType,
  runningTestsAtom,
  statusCountAtom,
  testStatusAtom,
  DoneTestStatusType,
  useRunHooks,
  showTestsAtom,
} from './testHooks'
import CustomErrorBoundary from '../../utils/ErrorFallback'
import {
  type WasmTestCase,
  type TestStatus,
  type WasmTestResponse,
} from '@gloo-ai/baml-schema-wasm-web/baml_schema_build'
import JsonView from 'react18-json-view'
import clsx from 'clsx'
import { Filter, Pin, Play } from 'lucide-react'
import { selectedFunctionAtom, selectedTestCaseAtom } from '../EventListener'

const TestStatusMessage: React.FC<{ testStatus: DoneTestStatusType }> = ({ testStatus }) => {
  switch (testStatus) {
    case 'passed':
      return <div className='text-vscode-testing-iconPassed'>Passed</div>
    case 'llm_failed':
      return <div className='text-vscode-testing-iconFailed'>LLM Failed</div>
    case 'parse_failed':
      return <div className='text-vscode-testing-iconFailed'>Parse Failed</div>
    case 'error':
      return <div className='text-vscode-testing-iconFailed'>Unable to run</div>
  }
}

const TestStatusIcon: React.FC<{ testRunStatus: TestStatusType; testStatus?: DoneTestStatusType }> = ({
  testRunStatus,
  testStatus,
}) => {
  return (
    <div className='text-vscode-descriptionForeground'>
      {
        {
          queued: 'Queued',
          running: <VSCodeProgressRing className='h-4' />,
          done: (
            <div className='flex flex-row items-center gap-1'>
              {testStatus && <TestStatusMessage testStatus={testStatus} />}
            </div>
          ),
          error: (
            <div className='flex flex-row items-center gap-1'>
              <div className='text-vscode-testing-iconFailed'>Unable to run</div>
            </div>
          ),
        }[testRunStatus]
      }
    </div>
  )
}

type FilterValues = 'queued' | 'running' | 'error' | 'llm_failed' | 'parse_failed' | 'passed'
const filterAtom = atom(new Set<FilterValues>(['running', 'error', 'llm_failed', 'parse_failed', 'passed']))

const checkFilter = (filter: Set<FilterValues>, status: TestStatusType, test_status?: DoneTestStatusType) => {
  if (filter.size === 0) {
    return true
  }

  if (status === 'done') {
    if (test_status === undefined) {
      return false
    }
    return filter.has(test_status)
  }

  return filter.has(status)
}

const LLMTestResult: React.FC<{ test: WasmTestResponse; doneStatus: DoneTestStatusType }> = ({ test, doneStatus }) => {
  const failure = test.failure_message()
  const llm_response = test.llm_response()
  const llm_failure = test.llm_failure()
  const parsed = test.parsed_response()

  const latencyMs = llm_response?.latency_ms ?? llm_failure?.latency_ms
  const client = llm_response?.client ?? llm_failure?.client
  const model = llm_response?.model ?? llm_failure?.model

  return (
    <div className='flex flex-col gap-1 w-full'>
      {failure &&
        !(doneStatus === 'parse_failed' || (doneStatus === 'llm_failed' && (llm_response || llm_failure))) && (
          <div className='text-xs text-vscode-errorForeground'>{failure}</div>
        )}
      {(llm_response || llm_failure) && (
        <div className='text-xs text-vscode-descriptionForeground w-full'>
          <div>
            Took <b>{latencyMs?.toString()}ms</b> using <b>{client}</b> {model && <>(model: {model})</>}
          </div>
          <div className='flex flex-row gap-2'>
            <div className='flex flex-col'>
              Raw LLM Response:
              <div className='px-1 py-2'>
                {llm_response && (
                  <pre className='whitespace-pre-wrap bg-vscode-input-background py-2 px-1 rounded-sm'>
                    {llm_response.content}
                  </pre>
                )}
                {llm_failure && (
                  <pre className='text-xs text-vscode-errorForeground whitespace-pre-wrap'>
                    <b>{llm_failure.code}</b>
                    <br />
                    {llm_failure.message}
                  </pre>
                )}
              </div>
            </div>
            {(doneStatus === 'parse_failed' || parsed !== undefined) && (
              <div className='flex flex-col'>
                Parsed LLM Response:
                <div className='px-1 py-2'>
                  {failure && <pre className='text-xs text-vscode-errorForeground whitespace-pre-wrap'>{failure}</pre>}
                  {parsed !== undefined && (
                    <JsonView
                      enableClipboard={false}
                      className='bg-[#1E1E1E] px-1 py-1 rounded-sm'
                      theme='a11y'
                      collapseStringsAfterLength={200}
                      matchesURL
                      src={JSON.parse(parsed)}
                    />
                  )}
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}

const TestRow: React.FC<{ name: string }> = ({ name }) => {
  const test = useAtomValue(testStatusAtom(name))
  const filter = useAtomValue(filterAtom)

  if (!checkFilter(filter, test.status, test.status === 'done' ? test.response_status : undefined)) {
    return null
  }

  return (
    <div className='flex flex-row gap-2 items-start group'>
      <TestCaseActions testName={name} />
      <div className='flex flex-col'>
        <div className='flex flex-row items-center gap-2 text-xs'>
          <b>{name}</b>
          <TestStatusIcon
            testRunStatus={test.status}
            testStatus={test.status === 'done' ? test.response_status : undefined}
          />
        </div>
        {test.status === 'error' && <div className='text-xs text-vscode-errorForeground'>{test.message}</div>}
        {test.status === 'done' && (
          <div className='text-xs text-vscode-descriptionForeground'>
            <LLMTestResult test={test.response} doneStatus={test.response_status} />
          </div>
        )}
      </div>
    </div>
  )
}

const FilterButton: React.FC<{ selected: boolean; name: string; count: number; onClick: () => void }> = ({
  selected,
  name,
  count,
  onClick,
}) => {
  return (
    <Badge
      className={`flex flex-row items-center gap-1 cursor-pointer ${
        selected ? '' : 'text-muted-foreground bg-vscode-button-backgroundHover'
      }`}
      onClick={onClick}
    >
      <span className='text-xs'>
        {name}: {count}
      </span>
    </Badge>
  )
}

const TestStatusBanner: React.FC = () => {
  const statusCounts = useAtomValue(statusCountAtom)

  const [filter, setFilter] = useAtom(filterAtom)

  const toggleFilter = (status: FilterValues) => {
    setFilter((prevFilter) => {
      const newFilter = new Set(prevFilter)
      if (newFilter.has(status)) {
        newFilter.delete(status)
      } else {
        newFilter.add(status)
      }
      return newFilter
    })
  }

  return (
    <div className='flex flex-row gap-2 items-center flex-wrap'>
      <Filter size={16} />
      <FilterButton
        selected={filter.has('queued')}
        name='Queued'
        count={statusCounts.queued}
        onClick={() => toggleFilter('queued')}
      />
      <FilterButton
        selected={filter.has('running')}
        name='Running'
        count={statusCounts.running}
        onClick={() => toggleFilter('running')}
      />
      <FilterButton
        selected={filter.has('error')}
        name='Error'
        count={statusCounts.error + statusCounts.done.error}
        onClick={() => toggleFilter('error')}
      />
      <FilterButton
        selected={filter.has('llm_failed')}
        name='LLM Failed'
        count={statusCounts.done.llm_failed}
        onClick={() => toggleFilter('llm_failed')}
      />
      <FilterButton
        selected={filter.has('parse_failed')}
        name='Parse Failed'
        count={statusCounts.done.parse_failed}
        onClick={() => toggleFilter('parse_failed')}
      />
      <FilterButton
        selected={filter.has('passed')}
        name='Passed'
        count={statusCounts.done.passed}
        onClick={() => toggleFilter('passed')}
      />
    </div>
  )
}

const TestResults: React.FC = () => {
  const [showTests, setShowTests] = useAtom(showTestsAtom)

  return (
    <div className='flex flex-col gap-2 px-1 w-full'>
      <div className='flex flex-row items-center gap-2'>
        <Badge
          className={clsx('cursor-pointer', showTests ? 'bg-vscode-button-backgroundHover text-muted-foreground' : '')}
          onClick={() => setShowTests(false)}
        >
          All Tests
        </Badge>
        <Badge
          className={clsx('cursor-pointer', showTests ? '' : 'bg-vscode-button-backgroundHover text-muted-foreground')}
          onClick={() => setShowTests(true)}
        >
          Test Results
        </Badge>
      </div>

      {showTests ? <TestResultContent /> : <TestCaseList />}
    </div>
  )
}

const TestCaseActions: React.FC<{ testName: string }> = ({ testName }) => {
  const [selectedTestCase, setSelectedTestCase] = useAtom(selectedTestCaseAtom)

  const { isRunning, run } = useRunHooks()

  return (
    <div className='flex flex-col gap-1'>
      <Button
        variant={'ghost'}
        size={'icon'}
        className='p-1 rounded-md w-fit h-fit bg-vscode-button-background text-vscode-button-foreground hover:bg-vscode-button-hoverBackground'
        disabled={isRunning}
        onClick={() => {
          run([testName])
        }}
      >
        <Play size={10} />
      </Button>
      {selectedTestCase?.name === testName ? (
        <Button
          variant={'ghost'}
          size={'icon'}
          className='p-1 rounded-md w-fit h-fit   bg-vscode-button-background
                  text-vscode-button-foreground
                  flex'
          disabled
        >
          <Pin size={10} />
        </Button>
      ) : (
        <Button
          variant={'ghost'}
          size={'icon'}
          className='p-1 rounded-md w-fit h-fit   hover:bg-vscode-button-background
                hover:text-vscode-button-foreground
                hidden
                group-hover:flex'
          onClick={() => {
            setSelectedTestCase(testName)
          }}
        >
          <Pin size={10} />
        </Button>
      )}
    </div>
  )
}

const TestCaseList: React.FC = () => {
  const allTestCases = useAtomValue(selectedFunctionAtom)?.test_cases ?? []
  const [filter, setFilter] = useState('')
  const testCases = useMemo(() => {
    return allTestCases.filter((t) => t.name.includes(filter) || t.inputs.some((i) => i.value?.includes(filter)))
  }, [allTestCases, filter])

  const [selectedTestCase, setSelectedTestCase] = useAtom(selectedTestCaseAtom)

  const { isRunning, run } = useRunHooks()

  return (
    <div className='flex flex-col gap-2 px-2 w-full h-full'>
      <div className='flex flex-row gap-2 items-center flex-wrap'>
        <Filter size={16} />
        <VSCodeTextField
          placeholder='Filter test cases'
          className='w-32 shrink'
          value={filter}
          onInput={(e) => {
            setFilter((e as React.FormEvent<HTMLInputElement>).currentTarget.value)
          }}
        />
        {isRunning ? (
          <VSCodeButton className='bg-vscode-statusBarItem-errorBackground' disabled onClick={() => {}}>
            Cancel Not Supported
          </VSCodeButton>
        ) : (
          <>
            <Button
              className='px-1 py-1 h-full text-xs whitespace-nowrap bg-red-500 rounded-sm bg-vscode-button-background text-vscode-button-foreground hover:bg-vscode-button-hoverBackground'
              disabled={testCases.length === 0}
              onClick={() => {
                run(testCases.map((t) => t.name))
              }}
            >
              <div className='flex flex-row gap-1 items-center'>
                <Play size={10} />
                Run {filter ? testCases.length : 'all'} tests
              </div>
            </Button>
          </>
        )}
        {filter && (
          <div className='text-xs text-muted-foreground'>{allTestCases.length - testCases.length} filtered out</div>
        )}
      </div>
      <hr className=' border-muted-foreground w-full' />
      <div className='flex flex-col gap-1 overflow-y-auto h-full'>
        {testCases.map((test) => (
          <div className='flex flex-row gap-2 items-start group'>
            <TestCaseActions testName={test.name} />
            <div
              key={test.name}
              className={clsx(
                'flex flex-col gap-1 p-1 w-full',
                selectedTestCase?.name !== test.name ? 'cursor-pointer hover:bg-vscode-input-background' : '',
              )}
              onClick={
                selectedTestCase?.name === test.name
                  ? undefined
                  : () => {
                      setSelectedTestCase(test.name)
                    }
              }
            >
              <div className='text-xs'>{test.name}</div>
              <TestCaseCard test_case={test} />
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}

const TestCaseCard: React.FC<{ test_case: WasmTestCase }> = ({ test_case }) => {
  return (
    <div
      className='flex flex-col gap-2 max-w-full text-xs text-left truncate 
      text-vscode-descriptionForeground '
    >
      <div className='whitespace-pre-wrap break-all'>
        <div className='flex flex-col'>
          {test_case.inputs.map((input) => (
            <div key={input.name}>
              <b>{input.name}:</b>
              {input.value !== undefined && (
                <JsonView
                  enableClipboard={false}
                  className='bg-[#1E1E1E] px-1 py-1 rounded-sm'
                  theme='a11y'
                  collapseStringsAfterLength={50}
                  collapseObjectsAfterLength={2}
                  matchesURL
                  src={JSON.parse(input.value)}
                />
              )}
              {input.error && (
                <pre className='break-words whitespace-pre-wrap w-full border-vscode-textSeparator-foreground rounded-md border p-0.5'>
                  {input.error}
                </pre>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

const TestResultContent: React.FC = () => {
  const testsRunning = useAtomValue(runningTestsAtom)
  return (
    <div className='flex flex-col gap-2 px-2 w-full h-full'>
      <TestStatusBanner />
      <hr className=' border-muted-foreground' />
      <div className='flex flex-col gap-1 w-full h-full overflow-y-auto'>
        {testsRunning.map((testName) => (
          <TestRow key={testName} name={testName} />
        ))}
      </div>
    </div>
  )
}

export default TestResults
