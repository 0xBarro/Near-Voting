import React from "react";
import { useEffect, useState } from "react";
import { getGifts, contribute_to_gift } from "./utils";
import './global.css'

export const Gift = props => {
    const [val, setVal] = useState(0)
    return <div className="gift">
        <a key={props.key} href={props.url}>URL</a> - Needed Tokens: {props.n_tokens_required} - Given: {props.current_tokens}<br></br> 
        <input type='number' min={0} onChange={(v) => setVal(v.target.value)}></input> <button onClick={v => contribute_to_gift(props.account, props.url, val)}> Contribute </button>
    </div>
}

export const GiftsList = props => {
    const [giftsList, setGitfsList] = useState(undefined);

    useEffect(()  => {
        console.log("Getting gifts for account: ", props.account)
        getGifts(props.account).then(g => {console.log("GIfts value: ", g); setGitfsList(g)})
        console.log(giftsList)
    }, [])

    return <div>
        <h3>  My Wishlist list  </h3>
        {(giftsList !== undefined) && giftsList.map(g => <Gift key={props.url} {...g} account={props.account}></Gift>)}
    </div>
}