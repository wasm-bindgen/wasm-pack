#!/usr/bin/env node

const { install } = require("./binary");
install().catch((e) => {
  console.error(e.message || e);
  process.exit(1);
});
