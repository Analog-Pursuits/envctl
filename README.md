# ENVCTL
envctl is for Environment Control,

used correctly, it checks every command you make, and applies it to any workflow configured for it.

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