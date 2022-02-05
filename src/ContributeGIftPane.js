import React from "react";
import { SelectAccount } from './SelectAccount'

export const ContributeGiftPane = props => {
    const [account, setAccount] = React.useState(undefined);
    console.log("Contrib")

    if (account === undefined) { 
        return <SelectAccount setAccount={setAccount}></SelectAccount> 
    }
    else {
        return <div>
            <h1> Contribute Gift to {account}</h1>
        </div>
    }
}