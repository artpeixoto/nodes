## Resumo

Nodes'n'Procs é um crate que provem uma infraestrutura rudimentar para execucao de processos concorrentes em ambientes embarcados.  
Sua arquitetura é baseada em Nós e Processos. Nós representam dados comuns ao sistema, e Processos representam calculos sobre estes nós.
Com essa descricao, é possivel interpretar um codigo escrito nessa infraestrutura como um diagrama, onde dados fluem através dos nós, para os processos. Isso torna especialmente simples de interpretar, criar e manipular sistemas muito interconectados.
