## Introdução

*Nodes & Procs* é um crate que provem uma infraestrutura rudimentar para execucao de processos concorrentes em ambientes embarcados.  
Sua arquitetura é baseada em Nós e Processos. Nós são dados comuns ao sistema, e Processos executam calculos sobre estes nós.
Com essa descricao, é possivel interpretar um codigo escrito nessa infraestrutura como um diagrama, onde dados fluem através dos nós, para os processos. Isso torna especialmente simples de interpretar, criar, extender e modificar manipular sistemas muito interconectados.

## Motivação

A inspiração para Nodes&Procs é o [ROS](https://www.ros.org). De fato, a idéia principal dele é ser uma espécie de "microROS", uma vez que o ROS original precisa executar em um desktop (cringe).

## TODO

Até o momento, Nodes&Procs ainda é rudimentar. Ele tem 
