oscilatticGame :: [[Double]] -> [[Double]] -> OpenGame
oscilatticGame payoffs1 payoffs2 mediator = [opengame|

inputs      : (i, j, m) ;
feedback    : () ;
operation   : mediatorAdvice "mediator" mediator ;
outputs     : advice ;
returns     : () ;

inputs      : (i, j, advice) ;
feedback    : () ;
operation   : correlatedDecision "team1" [0, 1] advice ;
outputs     : a1 ;
returns     : payoffs1 !! i !! j ;

inputs      : (i, j, advice) ;
feedback    : () ;
operation   : correlatedDecision "team2" [0, 1] advice ;
outputs     : a2 ;
returns     : payoffs2 !! i !! j ;

inputs      : (i, j, advice) ;
feedback    : () ;
operation   : ruleProposal "ruleProposalRepository" ;
outputs     : ruleProposal ;
returns     : () ;

inputs      : (i, j, ruleProposal) ;
feedback    : () ;
operation   : votingSystem "voteSystem" ;
outputs     : voteResult ;
returns     : () ;

inputs      : (i, j, voteResult) ;
feedback    : () ;
operation   : ruleUpdate "ruleUpdateSystem" ;
outputs     : updatedRules ;
returns     : () ;

|]