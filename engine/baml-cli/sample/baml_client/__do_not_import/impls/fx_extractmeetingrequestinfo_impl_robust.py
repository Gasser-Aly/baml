# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_main import Main
from ..functions.fx_extractmeetingrequestinfo import BAMLExtractMeetingRequestInfo
from ..types.classes.cls_attendee import Attendee
from ..types.classes.cls_conversation import Conversation
from ..types.classes.cls_meetingrequest import MeetingRequest
from ..types.classes.cls_meetingrequestpartial import MeetingRequestPartial
from ..types.classes.cls_message import Message
from ..types.enums.enm_usertype import UserType
from baml_lib._impl.deserializer import Deserializer


# Impl: robust
# Client: Main
# An implementation of .


__prompt_template = """\
Given a user is trying to schedule a meeting, extract the relevant
information from the query.

Context:
```
Today is {now}
```

Conversation:
```
{convo.display}
```

Output JSON:
{
  // Either an exact time, or a relative time. Use ISO 8601 Duration Format
  // when specifying a relative time (e.g. P1D for 1 day from now).
  "when": string | null,
  // Names or preferably emails of attendees.
  "attendees": string[],
  // What is the topic of the meeting?
  "topic": string | null
}

JSON:\
"""

__input_replacers = {
    "{convo.display}",
    "{now}"
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[MeetingRequestPartial](MeetingRequestPartial)  # type: ignore


def output_adapter(output: MeetingRequestPartial) -> MeetingRequest:
    # This is a pure python function we're importing from a python file
    # we've written (./py_baml_example/utils.py)
    
    from py_baml_example.utils import find_attendee_by_email
    
    attendees = [
        find_attendee_by_email(detail) if '@' in detail else Attendee(name=detail, email="unknown")
    
        # output is a special variable that contains the output of the
        # previous step (in this case, the LLM).
        for detail in output.attendees
    ]
    return MeetingRequest(
        time=output.time or "ASAP",
        attendees=attendees,
        topic=output.topic or "Meeting"
    )



@BAMLExtractMeetingRequestInfo.register_impl("robust")
async def robust(*, convo: Conversation, now: str) -> MeetingRequest:
    response = await Main.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(convo=convo, now=now))
    deserialized = __deserializer.from_string(response.generated)
    return output_adapter(deserialized)