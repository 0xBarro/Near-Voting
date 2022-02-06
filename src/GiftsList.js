import React from "react";
import { useEffect, useState } from "react";
import { getGifts } from "./utils";

export const GiftsList = props => {
    const [giftsList, setGitfsList] = useState();

    useEffect(()  => {
        console.log("Getting gifts for account: ", props.account)
        getGifts(props.account).then(g => setGitfsList(g))
        console.log(giftsList)
    }, [])

    return <div>
        <h3>  My gifts list  </h3>
    </div>
}