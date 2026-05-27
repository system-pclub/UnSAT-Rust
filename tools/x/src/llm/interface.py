from abc import ABC, abstractmethod


class LLM(ABC):
    """Minimal interface for language model completions."""

    @abstractmethod
    def complete(self, system: str, user: str) -> str:
        """Return a completion given a system prompt and a user message."""
        ...
