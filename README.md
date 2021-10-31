# simple calculator

command line simple calculator and library

## Context Free Grammar

### Left Recursion

```text
Goal -> Statement

Statement -> Expression

Expression -> Expression + Term | Expression - Term | Term ｜ + Term | - Term

Term -> Term * Factor | Term / Factor | Factor

Factor -> ( Expression ) | Factor ^ Factor| number
```

### Right Recursion

```text
Goal -> Statement

Statement -> Expression

Expression -> Term Expression' | + Term Expression' | - Term Expression'

Expression' -> + Term Expression' | - Term Expression' | ε

Term -> Factor Term'

Term' -> * Factor Term' | / Factor Term' | ε

Factor -> ( Expression ) Factor' | number Factor'

Factor' -> ^ Factor | ε
```
