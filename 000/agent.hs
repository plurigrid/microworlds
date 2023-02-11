{-# LANGUAGE DataKinds #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TupleSections #-}
{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances, FlexibleContexts, TemplateHaskell #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE QuasiQuotes #-}

module Examples.Auctions.DoubleAuction where

import OpenGames
import OpenGames.Preprocessor
import Examples.Auctions.AuctionSupportFunctions

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
    inputs    :  nameValuePair  ;
    feedback  :   ;
    operation :  dependentDecision name (const [0,20..100]) ;
    outputs   :  bid ;
    returns   :  setPayoff nameValuePair payments  ;
    :---------------------------:

    outputs   :  bid ;
    returns   :  payments  ;
  |]


-- Transforms the payments into a random reshuffling with reserve price
transformPaymentsWithReservePrice kPrice kSlots = [opengame|

   inputs    : (bids,reservePrice) ;
   feedback  :      ;

   :-----------------:
   inputs    : (bids,reservePrice) ;
   feedback  :      ;
   operation : forwardFunction (auctionPaymentResPrice noLotteryPayment kPrice kSlots 0) ;
   outputs   : payments ;
   returns   :      ;
   :-----------------:

   outputs   : payments ;
   returns   :      ;
  |]

 -- Transforms the payments into a random reshuffling without reserve price
transformPaymentsWithoutReservePrice kPrice kSlots = [opengame|

   inputs    : bids ;
   feedback  :      ;

   :-----------------:
   inputs    : bids ;
   feedback  :      ;
   operation : forwardFunction (auctionPayment noLotteryPayment reservePrice kPrice kSlots 0) ;
   outputs   : payments ;
   returns   :      ;
   :-----------------:

   outputs   : payments ;
   returns   :      ;
  |]

-- Double auction with reserve price
doubleAuctionWithReservePrice kPrice kSlots = [opengame|

   inputs    : reservePrice    ;
   feedback  :      ;

   :-----------------:
   inputs    :      ;
   feedback  :      ;
   operation : natureDrawsEnergyValue "Seller" ;
   outputs   :  sellerValue ;
   returns   :      ;

   inputs    :      ;
   feedback  :      ;
   operation : natureDrawsEnergyValue "Buyer" ;
   outputs   :  buyerValue ;
   returns   :      ;

   inputs    :  sellerValue    ;
   feedback  :      ;
   operation :  biddingStage "Seller" ;
   outputs   :  sellerBid ;
   returns   :  payments  ;

   inputs    :  buyerValue    ;
   feedback  :      ;
   operation :  biddingStage "Buyer" ;
   outputs   :  buyerBid ;
   returns   :  payments  ;

   inputs    :  ([("Seller",sellerBid),("Buyer",buyerBid)],reservePrice)  ;
   feedback  :      ;
   operation :   transformPaymentsWithReservePrice kPrice kSlots ;
   outputs   :  payments ;
   returns   :      ;
   :-----------------:

   outputs   :  payments    ;
   returns   :      ;
   |]

-- Double auction without reserve price
doubleAuctionWithoutReservePrice kPrice kSlots = [opengame|

   inputs    : bids ;
   feedback  :      ;

   :-----------------:
   inputs    : bids ;
   feedback  :      ;
   operation : forwardFunction (auctionPayment noLotteryPayment 0 kPrice kSlots 0) ;
   outputs   : payments ;
   returns   :      ;
   :-----------------:

   outputs   : payments ;
   returns   :      ;
  |]

-- Double auction with 2 players
doubleAuctionWith2Players kPrice kSlots = [opengame|

   inputs    :  ;
   feedback  :  ;

   :-----------------:
   inputs    :  ;
   feedback  :  ;
   operation : natureDrawsEnergyValue "Alice" ;
   outputs   :  aliceValue ;
   returns   :  ;

   inputs    :  ;
   feedback  :  ;
   operation : natureDrawsEnergyValue "Bob" ;
   outputs   :  bobValue ;
   returns   :  ;

   inputs    :  aliceValue ;
   feedback  :  ;
   operation : biddingStage "Alice" ;
   outputs   :  aliceBid ;
   returns   :  alicePayments ;

   inputs    :  bobValue ;
   feedback  :  ;
   operation : biddingStage "Bob" ;
   outputs   :  bobBid ;
   returns   :  bobPayments ;

   inputs    :  [("Alice",aliceBid),("Bob",bobBid)] ;
   feedback  :  ;
   operation : doubleAuctionWithoutReservePrice kPrice kSlots ;
   outputs   :  payments ;
   returns   :  ;
   :-----------------:

   outputs   :  payments ;
   returns   :  ;
   |]
