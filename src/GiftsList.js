import React from "react";
import { useEffect, useState } from "react";
import { getGifts } from "./utils";

export const GiftsList = props => {
    const gifts = getGifts(props.account)
}