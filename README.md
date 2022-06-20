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

### Fundraising And Income

Integrating donation and investment is a great way for social organizations to fundraise. Social projects can not only raise funds but also earn income on this platform, moreover, users pay and invest money not only for social organizations but also for themselves. The combination of temporary funding with income and self-oriented incentives with altruism is exponentially powerful.

### Connecting To Capitalism

Social workers work in the market where capitalism doesn't work well; that means it is hard for social organizations to get sophisticated as the *invisible hand* works. Donainvest integrates social works and capitalism. Then, that visualizes the needs of what the society really desires and how social projects make the social impact.

#### The Relation Between Social Impact and The Token Price

Token prices are in proportion to how much people want to support the project. Because there is no other incentives like dividends or ownerships. Although there is another incentive that is expectations of price increase in the future, it should depend on the social impact that the project makes. 

## How It Works
### Offering Tokens

Everyone can offer tokens easily because it is on [this protocol](https://github.com/kentomisawa/lfico). The project owner sets the value backing percentage. The rest of them are sent to the project owner as funds. Project owners must consider regulations because if unexpected huge loss of users' assets happens, the offered tokens may have to be regulated. To prevent from that, project owners should set the value backing rate relatively high.

### Donations

Users choose what percentage of tokens to donate when they buy. It's like fees on an exchange but how much users pay is up to them. The more donation, the more bonus value the purchased tokens will get if the price increases. 

### Hedging Risks

