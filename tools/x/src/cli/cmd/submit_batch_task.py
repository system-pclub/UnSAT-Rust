
from openai import OpenAI


def run(args):
    in_path = getattr(args, "in")
    
    
    client = OpenAI()

    # 1) Upload your JSONL input file (purpose must be "batch")
    batch_input_file = client.files.create(
        file=open(in_path, "rb"),
        purpose="batch",
    )
    print("uploaded file:", batch_input_file.id)  # :contentReference[oaicite:1]{index=1}

    # 2) Create the batch (completion_window currently only supports "24h")
    batch = client.batches.create(
        input_file_id=batch_input_file.id,
        endpoint="/v1/responses",      # or "/v1/chat/completions", etc. :contentReference[oaicite:2]{index=2}
        completion_window="24h",
        metadata={"description": "my batch job"},
    )
    print("batch id:", batch.id, "status:", batch.status)