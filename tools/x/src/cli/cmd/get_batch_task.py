import time
from openai import OpenAI

TERMINAL = {"completed", "failed", "expired", "cancelled"}

def get_counts(batch_obj):
    rc = getattr(batch_obj, "request_counts", None)
    if rc is None:
        return None
    # rc may be dict-like or object-like depending on SDK version
    if isinstance(rc, dict):
        return rc.get("total", 0), rc.get("completed", 0), rc.get("failed", 0)
    return getattr(rc, "total", 0), getattr(rc, "completed", 0), getattr(rc, "failed", 0)

    
    
def run(args):
    batch_id = getattr(args, "batch_id")
    
    
    client = OpenAI()
    while True:
        batch = client.batches.retrieve(batch_id)  
        counts = get_counts(batch)
        if counts:
            total, done, failed = counts
            pct = (done / total * 100.0) if total else 0.0
            print(f"status={batch.status}  done={done}/{total} ({pct:.1f}%)  failed={failed}")
        else:
            print(f"status={batch.status}")

        if batch.status in TERMINAL:
            out_text = client.files.content(batch.output_file_id).text
            with open(f"{batch_id}.jsonl", "w", encoding="utf-8") as f:
                f.write(out_text)
            break
        time.sleep(10)