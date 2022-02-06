import React from "react";
import { useEffect, useState } from "react";
import { getGifts } from "./utils";
import './global.css'

const Gift = props => {
    return <div className="gift">
        URL: {props.url} - Needed Tokens: {props.n_tokens_required}
    </div>
}

export const GiftsList = props => {
    const [giftsList, setGitfsList] = useState();

    useEffect(()  => {
        console.log("Getting gifts for account: ", props.account)
        getGifts(props.account).then(g => setGitfsList(g))
        console.log(giftsList)
    }, [])

    return <div>
        <h3>  My gifts list  </h3>
        {(giftsList !== undefined) && giftsList.map(g => <Gift {...g}></Gift>)}
    </div>
}