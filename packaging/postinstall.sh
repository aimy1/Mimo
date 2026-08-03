#!/bin/sh
# Grant CAP_NET_ADMIN capabilities to binary if setcap is available
if command -v setcap >/dev/null 2>&1; then
  setcap cap_net_admin+ep /usr/bin/mimo 2>/dev/null || true
fi
