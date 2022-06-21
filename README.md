# Donainvest

Donainvest is a donation-investment platform on which the value of offered tokens is in proportion to social goodness and social workers are able to continuously fundraise without crypto-related licenses.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

Additionally, if you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 8000.

## What It Does

### Fundraising Without a Crypto-Related License

Social workers can raise funds with [this protocol](https://github.com/kentomisawa/lfico), therefore everyone can easily offer tokens. 

### Donation And Investment

Integrating donation and investment is a great way for social organizations to fundraise. Users pay and invest money not only for social organizations but also for themselves. The combination of self-oriented investment and altruistic donation is exponentially powerful.

### Connecting To Capitalism

Social workers work in a market where capitalism doesn't work well; that means it is hard for social organizations to get sophisticated as the *invisible hand* works. Donainvest integrates social works and capitalism. Then, that visualizes the needs of what the society really desires and how social projects make social impact.

#### The Relation Between Social Impact and The Token Price

Token prices are in proportion to how much people want to support the project. Because there are no other incentives like dividends or ownerships. Although there is another incentive which is expectations of price increases in the future, it should depend on the social impact that the project makes. 

## How It Works

### Donations

Users choose what percentage of tokens to donate when they buy. It's like fees on an exchange but how much users pay is up to them. The more donation, the more bonus value the purchased tokens will get if the price increases.  
Project owners set the *Donation Bonus Power*; it is the variable to determine how much users get bonuses by their donations. There is a tradeoff between the token price and the amount of donation. If a project owner wants to earn more income from donations, the Donation Bonus Power should be set higher. On the other hand, if the project owner wants the token price to increase more, they should set the Donation Bonus Power lower. Project owners can change it anytime later. This may be like capital increase as companies do.

### Risk Hedging

Project owners have to hedge risks for users because unexpected asset loss causes restrictions on the purchase of the tokens. To prevent that, project owners set the *Value Guarantee Rate*. It is the rate of the values that users never lose related to the purchased values. Users also determine the rate to guarantee the part of held token values, but users cannot set the *Value Guarantee Rate* less than the project owner sets. This rate might be different from jurisdictions in which target users exist. Project owners must consider the *Value Guarantee Rate* is safe enough not to be regulated and to protect users from huge asset loss.

### Pricing

The issued tokens are a kind of NFTs. Because amounts of donations and *Value Guarantee Rate* are different for individuals; the more donation the more bonus value and the higher *Value Guarantee Rate* the less volatility. If a project owner makes the rate of donations and *Value Guarantee Rate* not optional to users, every issued token is at the same price. Although the volatility of purchased tokens issued by the same project owner could be different, the price users buy at is the same. Investment returns are differentiated by donation and *Value Guarantee Rate*. That seems hard to use those tokens, however, it is easy to pay with because they are coupled with the DEX and swappable.

#### Donation Bonuses and Investment Returns

There are two factors that users get values with, which are Donation Bonuses and Investment Returns. Donation Bonuses are derived from the amount of donation and *Donation Bonus Power*, Investment Returns are derived from the amount of purchased tokens and *Value Guarantee Rate*. Even though a price of tokens has increased by 10%, the values the users obtain might be not 10%; it could be 5% or 15%. It is various to users.

### KYC, AML/CFT, etc

Due to the exemption of crypto-related licenses for project owners, it's not able for anonymous users to transfer or exchange tokens. Users have to get permission from regulated organizations. If any exchanges don't allow a user to exchange, the user may not able to get an investment return, but still, offering tokens are feasible without any licenses. Because that is completely the same as selling a dynamic pricing product.
