import React, { useState, useEffect } from "react";
import { SelectAccount } from './SelectAccount'
import { Gift } from "./GiftsList";
import { getGifts } from "./utils";

export const ContributeGiftPane = props => {
    const [account, setAccount] = React.useState(undefined);
    const [giftsList, setGitfsList] = useState(undefined);

    useEffect(() => {
        if (account !== undefined) {

            console.log("Getting gifts for account: ", props.account)
            getGifts(account).then(g => setGitfsList(g))
            console.log(giftsList)

        }
    }, [account])

    if (account === undefined) { return <SelectAccount setAccount={setAccount}></SelectAccount> }

    else {
        return <div>
            <h1> Contribute Gift to {account}</h1>
            {(giftsList !== undefined) && giftsList.map(g => <Gift key={g.url} {...g}></Gift>)}
        </div>
    }
}