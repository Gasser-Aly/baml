from .baml_py import BamlError
from typing import Optional

# Define the BamlValidationError exception with additional fields
# note on custom exceptions https://github.com/PyO3/pyo3/issues/295
# can't use extends=PyException yet https://github.com/PyO3/pyo3/discussions/3838
class BamlValidationError(BamlError):
    def __init__(self, prompt: str, message: str, raw_output: str):
        super().__init__(message)
        self.prompt = prompt
        self.message = message
        self.raw_output = raw_output

    def __str__(self):
        return f"BamlValidationError(message={self.message}, raw_output={self.raw_output}, prompt={self.prompt})"

    def __repr__(self):
        return f"BamlValidationError(message={self.message}, raw_output={self.raw_output}, prompt={self.prompt})"

class BamlClientFinishReasonError(BamlError):
    def __init__(self, prompt: str, message: str, raw_output: str, finish_reason: Optional[str]):
        super().__init__(message)
        self.prompt = prompt
        self.message = message
        self.raw_output = raw_output
        self.finish_reason = finish_reason

    def __str__(self):
        return f"BamlClientFinishReasonError(message={self.message}, raw_output={self.raw_output}, prompt={self.prompt}, finish_reason={self.finish_reason})"

    def __repr__(self):
        return f"BamlClientFinishReasonError(message={self.message}, raw_output={self.raw_output}, prompt={self.prompt}, finish_reason={self.finish_reason})"
