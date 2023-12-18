# Montador e simulador Assembly RISC-V

## Integrantes
- Gabriel Borges - 202006401
- Arthur Souza - 200014978
- Marcelo Piano - 200049496
## Como executar
basta ter o Rust instalado e rodar o comando:
```bash
cargo run -- caminho-do-arquivo-assembly
```
## Exemplo
```bash
cargo run -- assembly/teste_simulador.txt
```
### Observações
Os registradores utilizados no programa Assembly devem estar na forma x0,x1...x31.
O montador reconhece apenas instruções da ISA RV32I.
O código assembly não pode conter comentários nem pseudoinstruções para ser executado corretamente.
## Documentação / Site do projeto
para acessar a documentação basta rodar o comando:
```bash
cargo doc --open
```
## Versão utilizada
cargo 1.74.1
## Histórico
A linguagem surgiu como projeto pessoal de um empregado da Mozilla, Graydon Hoare. A Mozilla começou a financiar o projeto em 2009, depois de perceber o potencial do mesmo. Só em 2010 é que fez um anuncio publico. No mesmo ano, em que o compilador inicial (escrito em OCaml) é rescrito para Rust. Conhecido como rustc, este consegui compilar-se a si próprio em 2011, usando o LLVM como back end.

A primeira versão pré-alpha numerada do compilador Rust ocorreu em Janeiro de 2012, mas só passado 3 anos, em 15 de Maio de 2015 é que foi lançada uma versão estável do mesmo.

Mesmo sendo um projeto patrocinado pela Mozilla, este é um open community project. Toda a gente pode participar de forma ativa no desenvolvimento e refinamento da linguagem, de diferente formas. Seja a melhorar a documentação, reportando BUGs, apresentando propostas de novas funcionalidades, melhorias ou mesmo desenvolvendo o compilador.

A linguagem levou um grande melhoramento através do feedback obtido pela experiência de desenvolvimento da escrita do Servo, um browser engine moderno e de elevada performance, desenhado para aplicações e uso embebido. Atualmente, grande parte dos commits são de membros da comunidade.

Rust ganhou o primeiro lugar da linguagem mais amada de 2016 num inquérito realizado pelo Stack Overflow.
## Premissas principais

Segurança e Confiabilidade: Rust foi projetada para garantir a segurança do código, eliminando muitos tipos comuns de erros de programação, como acessos inválidos à memória e condições de corrida.

Performance: A linguagem busca fornecer desempenho próximo ao código nativo, permitindo controle fino sobre os recursos de hardware, sem comprometer a segurança.

Concorrência: Rust facilita a construção de programas concorrentes e paralelos, garantindo segurança em operações multi-threaded.

## Usuários

O Rust é atraente para diversos perfis de desenvolvedores e para uma variedade de casos de uso:

Desenvolvedores de sistemas e aplicativos de baixo nível: Pode ser usado para criar desde sistemas operacionais até aplicações de rede e sistemas embarcados.

Web e Servidores: Com ferramentas modernas como o Rocket, Rust está se tornando popular para desenvolvimento web e para a construção de servidores seguros e de alto desempenho.

Jogos e Gráficos: Rust oferece um desempenho excepcional e é usado em algumas bibliotecas e engines de jogos.

## Domínio

O Rust é uma linguagem de programação de propósito geral, o que significa que pode ser aplicada em várias áreas:

Sistemas Operacionais: Rust é adequado para construir sistemas operacionais devido à sua segurança e desempenho.

IoT e Embarcados: Por ser uma linguagem de baixo nível com garantias de segurança, é uma escolha interessante para dispositivos IoT.

Ferramentas de Segurança: A linguagem é adotada para desenvolver ferramentas de segurança devido à sua capacidade de prevenir erros de programação comuns.

## Construtores

Em Rust, os "construtores" não são um recurso explícito como em algumas outras linguagens de programação, como Java ou Python, onde existe um método especial chamado "construtor". Em Rust, você pode criar uma estrutura de dados (como uma struct) e inicializá-la de algumas maneiras:

### Inicialização Direta:
Você pode criar uma instância da estrutura e definir os valores de seus campos imediatamente durante a criação:

Por exemplo, para uma estrutura Pessoa com campos nome e idade, você pode criar uma nova instância assim:

"Crie uma pessoa com o nome 'Alice' e idade 30."

### Métodos Associados:
É possível definir métodos dentro de um bloco impl para a estrutura. Esses métodos associados servem como construtores personalizados, permitindo a criação de instâncias da estrutura com valores específicos:

Por exemplo, para uma estrutura Pessoa:

"Crie uma função 'nova' que recebe um nome e uma idade, e retorne uma nova instância de Pessoa com esses valores."

### Macros:
Rust oferece macros que podem gerar código repetitivo. Você pode criar macros para criar instâncias de estruturas com valores pré-definidos:

Por exemplo, uma macro chamada pessoa! poderia ser usada para criar uma nova instância de Pessoa com valores passados como argumentos para a macro.

## Legibilidade

### Sintaxe Clara: 
A linguagem tem uma sintaxe organizada e consistente, facilitando a compreensão do código.

### Mensagens de Erro Explicativas: 
Quando há problemas, as mensagens de erro detalhadas ajudam a entender e corrigir os problemas mais facilmente.

### Nomenclatura Descritiva:
Rust incentiva nomes claros e autoexplicativos para variáveis e funções, melhorando a compreensão do código.

### Documentação Integrada:
A capacidade de incluir documentação diretamente no código facilita a compreensão do propósito e funcionamento de cada parte.

### Ferramentas de Formatação:
O Rustfmt mantém um estilo de código consistente, ajudando na legibilidade ao padronizar a formatação.

## Capacidade de escrita


### Expressividade e Abstração:
A linguagem permite escrever código de forma concisa e reutilizável usando recursos como traits e generics.

### Ferramentas e Ecossistema:
O Rust oferece ferramentas como Cargo, Rustfmt e Clippy, além de uma variedade de bibliotecas, facilitando a escrita e manutenção do código.

### Segurança e Feedback Rápido:
Embora priorize a segurança, Rust não compromete a produtividade, oferecendo feedback rápido durante o desenvolvimento para identificar erros.

### Curva de Aprendizado Inicial:
Inicialmente desafiador devido ao sistema de tipos robusto, mas a fluidez na escrita de código melhora à medida que se familiariza com os conceitos do Rust.

## Confiabilidade

### Segurança de Memória:
Rust previne segfaults, vazamentos de memória e erros de uso após liberação de memória (use-after-free).

### Concorrência Segura:
Evita condições de corrida e oferece segurança em operações concorrentes.

### Controle de Mutabilidade:
Regras claras sobre mutabilidade evitam alterações inesperadas nos dados.

### Análise Estática:
O compilador realiza verificações durante a compilação para identificar erros antes da execução do código.

### Práticas de Programação Segura:
Incentiva boas práticas para gerenciamento de memória, tratamento de erros e operações concorrentes.

### Desempenho Confiable:
Mantém um alto desempenho próximo ao código nativo, mesmo com suas garantias de segurança.

## Custo de Aprendizado:
### Curva de Aprendizado:
Rust pode ter uma curva de aprendizado íngreme, especialmente para programadores novos na linguagem. Entender o sistema de tipos, o conceito de propriedade (ownership) e as regras de empréstimo (borrowing) pode exigir tempo extra inicialmente.
## Custo de Desenvolvimento:
### Rigor na Segurança:
Rust exige um código mais seguro e robusto, o que pode levar mais tempo durante o desenvolvimento para garantir que o código siga as regras de propriedade, empréstimo e mutabilidade.

### Verificações Estáticas:
O compilador realiza verificações rigorosas durante a compilação, o que pode exigir ajustes e correções no código para atender aos requisitos de segurança.

## Custo de Desempenho:
### Overhead de Segurança:
Algumas medidas de segurança adicionais em Rust podem gerar um pequeno overhead em comparação com linguagens que não têm as mesmas garantias de segurança.
### Benefícios e Equilíbrio:
Trade-off: O custo inicial mais alto em termos de aprendizado e rigor na escrita de código é compensado pelos benefícios de segurança, confiabilidade e desempenho próximo ao código nativo que Rust oferece.
Em resumo, o "custo" em Rust pode envolver um período inicial de aprendizado mais desafiador, exigir práticas mais rigorosas de desenvolvimento para garantir a segurança do código e, em alguns casos, pode ter um pequeno overhead de desempenho. No entanto, esses custos são geralmente compensados pelos benefícios de segurança e confiabilidade que a linguagem proporciona.



