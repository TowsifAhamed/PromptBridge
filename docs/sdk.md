# Web SDK

`runVibeCode(options)` supports `protocol`, `http`, and `auto` modes.

```js
import { runVibeCode } from './sdk/vibecode-web.js'
await runVibeCode({ tool: 'clipboard', prompt: 'hello', mode: 'auto', http: { token } })
```

`auto` mode tries protocol first, waits ~800ms, then POSTs localhost fallback.
