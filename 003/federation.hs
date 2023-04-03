{-# LANGUAGE DataKinds #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TupleSections #-}
{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances, FlexibleContexts, TemplateHaskell #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE QuasiQuotes #-}

module Examples.Auctions.EnergyFederation where

import OpenGames
import OpenGames.Preprocessor
import Examples.Auctions.AuctionSupportFunctions
import Control.Monad.Random

----------
-- A Model
----------

---------------
-- 0 Parameters

type EnergyValue = Double
values = [0, 20..100]

reservePrice :: Double
reservePrice = 1

---------------------
-- 1 The actual games

-- Draws a value and creates a pair of _value_ _name_
natureDrawsEnergyValue name = [opengame|

    inputs    :   ;
    feedback  :   ;

    :-----:
    inputs    :   ;
    feedback  :   ;
    operation : nature (uniformDist values) ;
    outputs   : value ;
    returns   :  ;
    :-----:

    outputs   :  (name,value) ;
    returns   :    ;
  |]

-- Individual bidding stage
biddingStage name = [opengame|

    inputs    :  nameValuePair  ;
    feedback  :   ;

    :---------------------------:
