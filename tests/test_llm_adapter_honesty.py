import pytest

from global_workspace_runtime.cognition import LLMAdapter, LLMConfigurationError
from global_workspace_runtime.core.types import InternalState


def test_mock_mode_is_deterministic() -> None:
    llm = LLMAdapter(mode="mock")
    state = InternalState()

    first = llm.generate_candidates("analytic", {"text": "verify evidence"}, [], state, 3)
    second = llm.generate_candidates("analytic", {"text": "verify evidence"}, [], state, 3)

    assert [candidate.candidate_id for candidate in first] == [candidate.candidate_id for candidate in second]
    assert [candidate.text for candidate in first] == [candidate.text for candidate in second]
    assert all(candidate.evidence_refs == ["mock_runtime"] for candidate in first)


@pytest.mark.parametrize("mode", ["openai_compatible", "local"])
def test_non_mock_modes_fail_fast(mode: str) -> None:
    llm = LLMAdapter(mode=mode)

    with pytest.raises(LLMConfigurationError):
        llm.generate_candidates("analytic", {"text": "verify evidence"}, [], InternalState(), 1)