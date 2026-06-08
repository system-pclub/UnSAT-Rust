from openai import OpenAI


def run(args):
    batch_id = getattr(args, "batch_id")
    client = OpenAI()
    batch = client.batches.cancel(batch_id)
    print(f"batch id: {batch.id}  status: {batch.status}")
