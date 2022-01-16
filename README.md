# carbon
A scaffolding tool built in Rust. Create carbon copies of redundant code.

Relies on a simple configuration file that specifies the "rules" for your boilerplate code:

```
carbon-config.js

{
  "blueprints": [
    {
      "name": "test-module",
      "template": "src/blueprints/test-module.py",
      "root_dir": "tests",
      "file_type": ".js"
    }
  ]
}
```
