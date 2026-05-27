import os

from openai import OpenAI

from .interface import LLM


class OpenAILLM(LLM):
    """OpenAI chat-completions implementation of the LLM interface."""

    def __init__(
        self,
        model: str = "gpt-4o",
        api_key: str | None = None,
        temperature: float = 0.0,
    ) -> None:
        api_key = api_key or os.environ.get("OPENAI_API_KEY")
        if not api_key:
            raise RuntimeError(
                "OpenAI API key not found. Set the OPENAI_API_KEY environment variable "
                "or pass api_key explicitly."
            )
        self._client = OpenAI(api_key=api_key)
        self._model = model
        self._temperature = temperature

    def complete(self, system: str, user: str) -> str:
        response = self._client.chat.completions.create(
            model=self._model,
            temperature=self._temperature,
            messages=[
                {"role": "system", "content": system},
                {"role": "user", "content": user},
            ],
        )
        content = response.choices[0].message.content
        return content if content is not None else ""
