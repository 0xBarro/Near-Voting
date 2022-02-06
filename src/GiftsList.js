import React from "react";
import { useEffect, useState } from "react";
import { getGifts } from "./utils";

export const GiftsList = props => {
    const [giftsList, setGitfsList] = useState();
    const gifts = getGifts(props.account)
    return <div>
        <h3>  My gifts list  </h3>
    </div>
}