#!/usr/bin/env node

const { run } = require("./binary");
run().catch((e) => {
  console.error(e.message || e);
  process.exit(1);
});
