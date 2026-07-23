import os
from pathlib import Path

from openai import OpenAI

from .interface import LLM


def _load_openai_api_key_from_dotenv() -> str | None:
    """Load OPENAI_API_KEY from a nearby .env file without printing secrets."""
    for directory in (Path.cwd(), *Path.cwd().parents):
        dotenv = directory / ".env"
        if not dotenv.is_file():
            continue
        try:
            for raw_line in dotenv.read_text(encoding="utf-8").splitlines():
                line = raw_line.strip()
                if not line or line.startswith("#") or "=" not in line:
                    continue
                key, value = line.split("=", 1)
                if key.strip() != "OPENAI_API_KEY":
                    continue
                value = value.strip().strip("'\"")
                return value or None
        except OSError:
            continue
    return None


class OpenAILLM(LLM):
    """OpenAI chat-completions implementation of the LLM interface."""

    def __init__(
        self,
        model: str = "gpt-5.4-mini",
        api_key: str | None = None,
        temperature: float = 0.0,
    ) -> None:
        api_key = (
            api_key
            or os.environ.get("OPENAI_API_KEY")
            or _load_openai_api_key_from_dotenv()
        )
        if not api_key:
            raise RuntimeError(
                "OpenAI API key not found. Set the OPENAI_API_KEY environment variable "
                "or pass api_key explicitly."
            )
        self._client = OpenAI(api_key=api_key)
        self._model = model
        self._temperature = temperature

    def complete(self, system: str, user: str) -> str:
        if os.environ.get("UNSAT_LLM_DEBUG_PROMPTS"):
            print(f"[LLM] system prompt:\n{system}\n", flush=True)
            print(f"[LLM] user prompt:\n{user}\n", flush=True)
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
