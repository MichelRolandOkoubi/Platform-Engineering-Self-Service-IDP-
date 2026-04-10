from fastapi import FastAPI

app = FastAPI(title="ML Cost Engine")

@app.get("/health")
def health():
    return {"status": "ok"}

@app.post("/predict")
def predict(data: dict):
    # TODO: Implement prediction logic
    return {"prediction": 0.0}
