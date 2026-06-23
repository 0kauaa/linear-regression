# Linear Regression

Implementação da Regressão Linear Simples em múltiplas linguagens de programação.

O objetivo deste projeto é estudar como diferentes linguagens expressam um mesmo algoritmo estatístico, comparando aspectos como legibilidade, paradigma de programação, sistema de tipos, modelagem matemática, desempenho e ergonomia de desenvolvimento.

Atualmente o projeto possui implementações em:

- Haskell
- Rust

## Objetivo

A regressão linear simples busca encontrar a reta que melhor descreve a relação entre duas variáveis numéricas.

O modelo possui a forma:

$$
y = \beta_{1}x + \beta+{0}
$$

onde:

- $\beta_{1}$ é o coeficiente angular (inclinação da reta);
- $\beta_{0}$ é o intercepto;
- $x$ é a variável independente;
- $y$ é a variável dependente.

Os coeficientes são calculados pelo método dos mínimos quadrados.

### Coeficiente Angular

$\beta_{1} = \frac{n \sum{xy} \sum{y}}{n \sum{x^2} - (\sum{x})^2}$

### Intercepto

$$
\beta_{0} = \frac{\sum{y} - \beta_{1} \sum{x}}{n}
$$

## Estrutura do Projeto

```text
linear-regression/
├── haskell/
│   ├── app/
│ 	│   └── Main.rs
│   └── src/
│       └── LinearRegression.hs
│
└── rust/
    └── src/
    	├── linear_regression.rs
        └── main.rs
```

Cada diretório contém uma implementação independente do algoritmo.

## Como Executar

### Haskell

```bash
stack run
```

### Rust

```bash
cargo run
```
