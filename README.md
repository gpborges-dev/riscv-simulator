# Título do Projeto
Montador e simulador Assembly RISC-V.
## Para testar
cargo run -- caminho-do-arquivo-assembly
### Exemplo
cargo run -- assembly\teste_simulador.txt
## Documentação / Site do projeto
cargo doc --open
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

Em Rust, os "construtores" não são um recurso explícito como em algumas outras linguagens de programação, como Java ou Python, onde existe um método especial chamado "construtor".

Em Rust, você pode criar uma estrutura de dados (como uma struct) e inicializá-la de algumas maneiras:

Inicialização Direta:
Você pode criar uma instância da estrutura e definir os valores de seus campos imediatamente durante a criação:

Por exemplo, para uma estrutura Pessoa com campos nome e idade, você pode criar uma nova instância assim:

"Crie uma pessoa com o nome 'Alice' e idade 30."

Métodos Associados:
É possível definir métodos dentro de um bloco impl para a estrutura. Esses métodos associados servem como construtores personalizados, permitindo a criação de instâncias da estrutura com valores específicos:

Por exemplo, para uma estrutura Pessoa:

"Crie uma função 'nova' que recebe um nome e uma idade, e retorne uma nova instância de Pessoa com esses valores."

Macros:
Rust oferece macros que podem gerar código repetitivo. Você pode criar macros para criar instâncias de estruturas com valores pré-definidos:

Por exemplo, uma macro chamada pessoa! poderia ser usada para criar uma nova instância de Pessoa com valores passados como argumentos para a macro.

## Legibilidade


