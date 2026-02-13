import express from "express";
import { router } from "./routes";

const app = express();
app.use(router);

app.listen(4000, () => {
  console.log("Aureon Solana API running on :4000");
});
