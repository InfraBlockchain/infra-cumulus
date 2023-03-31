window.SIDEBAR_ITEMS = {"struct":[["ParentAsUmp","Xcm router which recognises the `Parent` destination and handles it by sending the message into the given UMP `UpwardMessageSender` implementation. Thus this essentially adapts an `UpwardMessageSender` trait impl into a `SendXcm` trait impl."],["TakeFirstAssetTrader","Charges for execution in the first multiasset of those selected for fee payment Only succeeds for Concrete Fungible Assets First tries to convert the this MultiAsset into a local assetId Then charges for this assetId as described by FeeCharger Weight, paid balance, local asset Id and the multilocation is stored for later refund purposes Important: Errors if the Trader is being called twice by 2 BuyExecution instructions Alternatively we could just return payment in the aforementioned case"],["XcmFeesTo32ByteAccount","XCM fee depositor to which we implement the TakeRevenue trait It receives a Transact implemented argument, a 32 byte convertible acocuntId, and the fee receiver account FungiblesMutateAdapter should be identical to that implemented by WithdrawAsset"]],"trait":[["ChargeWeightInFungibles","ChargeWeightInFungibles trait, which converts a given amount of weight and an assetId, and it returns the balance amount that should be charged in such assetId for that amount of weight"],["PriceForParentDelivery",""]]};