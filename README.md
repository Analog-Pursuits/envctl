# ENVCTL

envctl is for Environment Control,

used correctly, it checks every command you make, and applies it to any workflow configured for it.

## Installation and Getting Started

You can find the documentation for installation and getting started here.

[Installation](https://github.com/Analog-Pursuits/envctl/blob/main/docs/getting-started/install.md)

[Getting Started](https://github.com/Analog-Pursuits/envctl/blob/main/docs/getting-started/getting-started.md)

# Rule configuration example

```json
[
  {
    "WorkflowName": "Kube",
    "Matches": "(kubectl (apply|delete))"
    "Rules": [
      {
        "RuleName": "Production-Block",
        "ErrorMessage": "Sorry, but you do not have access to Production",
        "ErrorType": "Error",
        "RuleExpressionType": "DSL",
        "Expression": "if k8s.config.current-cluster == 'Production'"
      },
      {
        "RuleName": "WASM-test",
        "RuleExpressionType": "WASM",
        "ruleset": "~/.envctl/rules/wasm-test.wasm"
      }
    ]
  }
]
```
