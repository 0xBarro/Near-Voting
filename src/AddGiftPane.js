import React, { useState } from "react";
import { insertGift } from "./utils";

export const AddGiftPane = props => {
    const [url, setUrl] = useState("")
    const [price, setPrice] = useState(0)

    return <div>
        <h1> Add Gift</h1>

        <form>
            <label htmlFor="url"> Gift URL </label><br></br>
            <input type="text" id="Url" onChange={(v) => setUrl(v.target.value)}></input><br></br>

            <label htmlFor="price"> Price (Near Tokens) </label><br></br>
            <input type="number" id="price" min={0}  onChange={(v) => setPrice(v.target.value)}></input><br></br>
            
            <label htmlFor="sub"></label><br></br>
        </form>
        <button onClick={() => {
            alert("Gift submitted")
            insertGift({url: url, n_tokens_needed: price})
        }}>Submit</button>
    </div>
}