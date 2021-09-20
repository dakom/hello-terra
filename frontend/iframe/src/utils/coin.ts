import {Coin} from "@terra-money/terra.js";

//The JSON serialized version of cosmwasm_std::Coin
interface NativeCoin {
    denom: string,
    amount: string
}

export function convertCoin({denom, amount}:NativeCoin):Coin {
    return new Coin(denom, amount);
}