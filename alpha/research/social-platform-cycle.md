# The Cycle of VC-Funded Social Platforms: Network Effects, the Recurring Rug-Pull, and What It Teaches About Breaking the Pattern

author: Research agent (commissioned)

date: 2026-06-13

status: draft for review

---

## A note on method and labels

This document distinguishes three kinds of statement throughout.

- **Documented history** is sourced to a citable event, figure, date, or quote.

- **Analysis** is the agent's synthesis of that history.

- **Commissioner's thesis** is the argument this work was commissioned to develop and test, flagged as such so it is never smuggled in as fact.

Claims that are thin, inferred, or that I could not verify to a primary or strong secondary source are marked `[UNVERIFIED]`.

One correction up front, because the brief is built partly on it. The brief calls for analysis of **"Discord's recent IPO."** As of June 2026, Discord has *not* completed an IPO. Reuters reported that Discord confidentially filed for a U.S. IPO on January 6, 2026, working with Goldman Sachs and JPMorgan, but the company remained privately held and not publicly traded as of mid-2026. The analysis below treats Discord as a pre-IPO case (a platform on the runway, not one that has taken off), which is arguably *more* useful for the thesis because it captures the moment of decision rather than the aftermath.

---

## Executive summary

There is a pattern, and it is old. Across more than three decades, people have used networked tools to build community, the community has generated real value through network effects, and that value has repeatedly been extracted, enclosed, or destroyed. The mechanism of loss varies (acquisition, shutdown, API enclosure, monetization-of-the-user, repurposing for an owner's agenda) but the shape recurs from Usenet and GeoCities through MySpace, Digg, LiveJournal, Yahoo's Groups and Chat and acquisition graveyard, Google's serial failures (including Reader), Vine, Reddit, Facebook, and Twitter, to Discord poised at the edge of the public markets. Niche cases (the forum Something Awful, the MMO City of Heroes), a non-rug-pull acquisition (Flickr's rescue by SmugMug), and the rare durable successes (Wikimedia, Signal, the fan-built Archive of Our Own, and the Stocksy co-op) sharpen the mechanics further.

The central analytical lens is Cory Doctorow's **enshittification**: platforms are first good to users, then abuse users to favor business customers, then claw back value from everyone, then die. But enshittification is a description of *behavior*, not its root cause. The deeper diagnosis this document argues for is a **capital-structure** one: venture funding requires an exit, an exit (acquisition or IPO) requires an extraction story, and extraction is the rug-pull. The technology of lock-in (captured social graph, non-portable identity, un-owned data) supplies the *leverage* that makes extraction possible without losing the users. So the cycle is driven by the marriage of an extraction imperative to an inability to leave.

The GeoCities and Yahoo Groups cases sit slightly apart and are the emotional core of the story: these were not exploited so much as *erased*. Decades of collective memory were deleted because users never owned their own data. That is the cleanest argument that data custody is not an ideological nicety but a defense against a community ceasing to exist at a company's convenience.

The antidotes (open protocols, cooperatives, nonprofit/foundation governance, portability law) have a mixed record. Most failed on funding, UX, or fragmentation. The rare durable survivors (Wikimedia, Signal) are notable precisely because they solved *funding and governance* unusually, not because they shipped better software. The synthesis: breaking the cycle requires coupling durable governance and legal structure (an ownership form that cannot be sold out, portable identity and data, open protocol) **with** the user experience and brand stability the community side has historically failed to deliver. You need both, and you must commit to the governance half at inception, because the history is clear that you cannot retrofit it after taking growth capital.

---

## Part I: The chronological narrative

### The pre-web roots (1980s–1990s): community as something you joined, not something you owned

**The WELL and the idea of virtual community.** The Whole Earth 'Lectronic Link, founded in 1985 by Stewart Brand and Larry Brilliant, became the touchstone for early online community ideals. Howard Rheingold's 1993 book *The Virtual Community* drew its central framing from his years on the WELL, arguing that people were forming real communities of feeling and mutual aid in text-only spaces. The WELL also surfaced the question that would recur for forty years: who owns and controls the space where a community lives? The community was real, but the servers, the member list, and the archive belonged to an organization, not the members. [Documented history; Rheingold's *The Virtual Community*, 1993.]

**Usenet and Eternal September.** Usenet, created in 1980, was the early distributed-community model: a federated, no-central-owner discussion system. Its culture had a famous rhythm. Every September a wave of new university students would arrive, breach the norms, and gradually be acclimated. In September 1993, AOL began offering its users Usenet access, and the influx never stopped. The community coined **"Eternal September"** (also "the September that never ended") for the moment when scale permanently outran the culture's capacity to absorb newcomers. It is an origin-myth of community degradation through growth, and notably it happened to a system *no one owned* and *no one could sell*, which complicates any tidy story that ownership is the only failure mode. Usenet declined over the following decades as spam, binary-file abuse, and the migration to the web hollowed it out. [Documented history. Analysis: the Usenet case shows decay can be cultural and structural, not only extractive.]

**AOL and the original walled garden.** Through the 1990s, AOL was the dominant on-ramp to being online for tens of millions, and it was a curated, proprietary enclosure: AOL Keywords, AOL chat rooms, AOL content, all inside the walls. The open web eventually routed around it. The lesson users absorbed, in retrospect, was that the convenient walled garden and the open network have different survival properties, and that being inside the garden means living by the gardener's rules. [Documented history; analysis.]

**Forums, mailing lists, IRC.** Beneath the brand-name services ran older infrastructures: phpBB and vBulletin forums, Listserv and Mailman mailing lists, and IRC for real-time chat. These were often self-hosted or community-run, which made them resilient to any single company's decisions but fragile to maintainer burnout and funding gaps. Much of what later platforms monetized was first built, unpaid, on this infrastructure. [Documented history; analysis.]

### GeoCities: the clean early rug-pull (1994–2009)

GeoCities, launched in beta around 1994, let ordinary people build personal web pages organized into themed "neighborhoods." At its peak it hosted on the order of 38 million pages built by roughly 3 million users `[UNVERIFIED: the 38M-pages / 3M-users figures are widely cited but I did not confirm them against a GeoCities or Yahoo primary source]`, and at the time of its acquisition it was among the most popular sites on the internet. Yahoo bought it in 1999 for roughly $3.6 billion in stock. [Documented history.]

In April 2009 Yahoo stopped new registrations, and on October 26, 2009, it shut GeoCities down and deleted the content. Archive Team, the volunteer digital-preservation group, began the GeoCities rescue project in April 2009 and worked through to the October 26 closing date. The rescued data was eventually released as a torrent on the order of 900GB. The framing from Archive Team's founder Jason Scott was blunt: this was deletion of history, on purpose, at scale. [Documented history.]

GeoCities is the clean early instance of the full arc: community-built value, acquisition, then erasure. Crucially, the loss was not exploitation; it was deletion. Families lost photo albums, fan communities lost their archives, because none of them owned the data. [Analysis.]

### Friendster and MySpace: the first network-effect winners to lose (2002–2011)

**Friendster** (2002) was an early social-network leader that famously buckled under its own growth, with performance problems and a culture migration draining it before it was eventually repositioned and faded. It is the first clear case of network-effect value evaporating rather than being extracted. [Documented history; `[UNVERIFIED]` on the precise technical-vs-cultural weighting of its decline.]

**MySpace** rose to dominance around 2005–2008 and was acquired by Rupert Murdoch's News Corporation in 2005 for about $580 million. Under corporate ownership it stagnated, lost the migration race to Facebook, and declined sharply; News Corp later sold it in 2011 for roughly $35 million, a fraction of the purchase price. In 2019 MySpace acknowledged it had lost virtually all music uploaded between roughly 2003 and 2015 (reportedly more than 50 million songs) during a server migration, another instance of community-contributed content destroyed by custodial failure. [Documented history; `[UNVERIFIED]` on the exact "50 million songs / 12 years" figures, which trace to MySpace's own statement.]

### Digg: the self-inflicted rug-pull, and the refuge that became the next rug-puller (2010)

Digg adds a mechanic the others miss: a platform that rug-pulled its *own* community through a redesign, not an acquisition or a sunset. In August 2010 Digg launched "v4," which deemphasized user-submitted content in favor of publisher-contributed content, and users felt the company was selling out to the mainstream media it had originally set out to replace. The v4 redesign also removed the ability to "bury" stories, a core self-regulation feature, and let media companies auto-submit content, which the grassroots community experienced as a betrayal. The revolt was organized and symbolic. On "Quit Digg Day," August 30, 2010, users flooded Digg's own front page with links automatically submitted from Reddit, and left en masse for Reddit. Digg's traffic fell by roughly a quarter in the following month while Reddit's grew 230% over 2010. [Documented history.]

The detail that matters most for this narrative is the recursion. The community fled Digg *to Reddit*, which offered the user-submission, community-moderation model Digg had just abandoned. Thirteen years later, Reddit ran its own version of the same play (the 2023 API enclosure and the "landed gentry" episode covered below). The refuge from one rug-pull became the site of the next. This is the cycle's clearest self-demonstration: there is no safe harbor that is merely a better-behaved company, because the same structural pressures eventually reach it too. [Analysis; commissioner's thesis.]

### LiveJournal: acquisition, then repurposing under a new legal regime (2007–2017)

LiveJournal is the early parallel to Twitter/X: a community bought, then bent to an owner's jurisdiction and agenda. Six Apart sold LiveJournal to the Russian company SUP Media in 2007; it kept operating from a California subsidiary but began moving operations to Russia from 2009. The decisive move came later. In December 2016 LiveJournal relocated its servers from California to Russia, taking millions of pages of fiction, fan-fiction, and personal writing with them, and triggering a user migration in protest and out of security fears. In April 2017 a new terms of service classified any blog exceeding 3,000 daily viewers as a media outlet subject to Russian content law, and pro-Ukrainian and dissident blogs began disappearing. A former SUP advisor captured the stakes: he said the servers had moved "closer" not to the platform's authors and readers but to those who wanted to monitor them, and the updated agreement forbade posting "political solicitation materials" without permission. [Documented history.]

LiveJournal also illustrates the *exit* half of the thesis better than almost any other case, because a genuine escape route existed: **Dreamwidth**, an open-source fork of the LiveJournal codebase, let users import their entire history (posts, comments, tags, icons) and leave. When the servers moved, users circulated Dreamwidth's import instructions as a way to take the whole archive elsewhere. The lesson is precise: because the code was forkable and an importer existed, the community had somewhere to go that it actually owned. Most rug-pulled communities had no such option. [Documented history; analysis; commissioner's thesis that a real exit disarms the rug-pull.]

### Yahoo as serial rug-puller: Groups, Chat, and the acquisition graveyard (1997–2020)

Yahoo is the single richest illustration that the pattern is not incidental, because one company rug-pulled communities through at least three distinct mechanisms: quiet deletion (Groups), liability-and-decay collapse (Chat), and acquire-then-neglect (Flickr, Delicious, Upcoming, and others). Treating them together is more instructive than treating Groups alone.

**Yahoo Groups: deletion as a business decision (2001–2020).** Yahoo Groups ran for about two decades as forums-plus-mailing-lists for an enormous range of communities. Then it was wound down in stages. Yahoo removed online access to discussions and most features on February 1, 2020, turning groups into bare mailing lists, and on October 13, 2020, announced a complete shutdown set for December 15, 2020. The deletion of user content had come earlier: in October 2019 Yahoo announced all posted content would be deleted, a date that moved from December 2019 to January 31, 2020. The official reason given was a decline in usage and a focus on "premium, trustworthy content." The scale of what was at stake is captured by the rescue effort. Archive Team's initial crawl found nearly 1.5 million groups with public message archives, an estimated 2.1 billion messages between them, and by late October 2019 they had archived around 1.8 billion of those public messages. Files, photos, and attachments behind member-only logins were much harder to save. [Documented history.]

**Yahoo Chat: collapse by liability and decay (1997–2012).** Yahoo Chat is a separate service from Groups, and its arc shows a different mechanic: the community destroyed as collateral damage in an advertiser-and-liability panic, then left to rot. In June 2005, after a Houston television station reported that some user-created Yahoo chat rooms were being used to solicit sex with minors, advertisers including Pepsi and Georgia-Pacific canceled advertising, and Yahoo pulled the plug on possibly hundreds of user-created chat rooms. The crackdown swept up hundreds of innocent rooms that had nothing to do with the abuse, leaving ordinary communities suddenly without the spaces they relied on, while Yahoo-created rooms stayed open. In October 2005, under an agreement with the New York and Nebraska attorneys general, Yahoo barred rooms promoting adult-child sex, eliminated the teen chat category entirely, and restricted all chat rooms to users 18 and older. The rooms never recovered. Yahoo announced on November 30, 2012 that public chat rooms would be discontinued as of December 14, 2012, framing it as a way to refocus on core products; by 2007 it had already been estimated that at least 75% of users in Yahoo chat rooms were bots. The longer tail is the same abandonment story: Yahoo Messenger itself shut down entirely on July 17, 2018, and its stopgap replacement Yahoo Together was discontinued on April 4, 2019. [Documented history.]

The Chat case matters because the trigger was not a quiet strategic decision but the collision of unmoderated user content with advertiser tolerance and legal liability. The value users built was contingent on Yahoo's willingness to host the liability of hosting it, and once advertisers fled, the rooms (legitimate and illegitimate alike) were gone. Online ads were most of Yahoo's revenue, which is exactly why advertiser flight was decisive. [Analysis.]

**The acquisition graveyard: acquire, neglect, abandon (2005–2014).** Yahoo also ran a repeatable pattern with the community services it bought. The 37signals analysis put it plainly: whether Flickr, Delicious, MyBlogLog, or Upcoming, the post-purchase story was similar, with founders departing around the two-year mark, products stalling, and customers left holding the bag. Flickr was acquired in March 2005 for about $35 million, and Delicious in December 2005 for roughly $15–20 million, with Yahoo promising to preserve each community. Delicious then went stagnant and its founder Joshua Schachter said he was stripped of responsibilities within a year of acquisition. In December 2010 a leaked internal slide listed Delicious and others for "sunset." Upcoming.org, acquired in 2005, was let to stagnate after its founder left, and Yahoo announced its closure with about eleven days' notice, prompting Andy Baio to ask Archive Team for an emergency rescue that saved its catalogue of events. Baio's own verdict on selling to Yahoo, recorded in the earlier research, was that it was "a horrible mistake." [Documented history; `[UNVERIFIED]` on the exact Baio quote wording.]

Notably, Upcoming has a rare partial-exit ending. After Yahoo shelved it, Archive Team backed up the entire site, and a Yahoo contact later intervened in the domain auction to sell the name back to Baio, who relaunched it as an independent side project. That is the exception that proves the rule: recovery depended on volunteer archivists plus a lucky personal intervention, not on any structural right the community held. [Analysis.]

Together, Groups, Chat, and the acquisition cluster make the content-custody argument more sharply than any single exploitation case: the harm was rarely that users were monetized, and more often that decades of collective memory were destroyed or abandoned, because the company owned the archive and the users did not. [Analysis; commissioner's thesis that data custody is a survival question, not a values nicety.]

**The counter-example: Flickr's rescue by SmugMug (2018).** Not every acquisition is a rug-pull, and honesty requires the exception. In April 2018 the family-owned, privately held SmugMug bought Flickr out of Verizon's Oath division (the remnant of Yahoo), with CEO Don MacAskill saying he wanted to preserve the photography community. The framing was an explicit rejection of the surveillance model: MacAskill said SmugMug does not mine customers' photos to sell to the highest bidder or turn into targeted advertising, a pointed contrast with the contemporaneous Cambridge Analytica scandal. But the rescue was not free of pain or risk. In November 2018 Flickr ended the free 1TB storage tier and capped free accounts at 1,000 photos, while restoring unlimited storage for paid Pro users, on the reasoning that a free terabyte attracted users who could not sustain the service. And the model stayed precarious: in December 2019 SmugMug emailed subscribers an urgent request to help find more paying users. The lesson cuts two ways for the thesis. A privately held, non-VC, founder-controlled owner that refuses data monetization *can* be a genuinely better steward (no extraction imperative from public markets). But "better owner" is not the same as "structural protection," because the community's fate still rests on one company's continued solvency and goodwill, not on anything the users own. A good king is still a king. [Documented history; analysis.]

### Google's serial social failures: the graveyard and what it taught (2009–2019)

Google tried repeatedly to build social and repeatedly killed what it built.

- **Google Wave** (2009) launched with enormous hype as a real-time collaboration-and-conversation tool, confused nearly everyone about what it was for, and was discontinued within about a year.

- **Google Buzz** (2010) launched into Gmail and immediately created a privacy scandal: it auto-exposed users' most-emailed contacts as a public follower list, drawing an FTC complaint that Google settled in 2011 with a consent decree requiring privacy audits. [Documented history; `[UNVERIFIED]` on exact settlement terms.]

- **Google+** (2011) was the big, sustained attempt to challenge Facebook. It never achieved comparable engagement, and Google announced its consumer shutdown in 2018 (accelerated after disclosure of an API bug exposing user data), winding it down in 2019.

- **Google Reader** (shut down July 1, 2013) is the cleanest case because, unlike Wave and Buzz, it was loved and it worked. Within hours of the March 2013 announcement, a Change.org petition gathered tens of thousands of signatures (more than 46,000), with its creator writing that confidence in Google's other products required trusting Google not to nuke the ones people relied on. Google's stated reasons were declining usage and a desire to pour energy into fewer products, but the blunt fact was that Reader made no money and cost money to run. The thesis-relevant detail: a 2011 redesign had already stripped Reader's social/sharing features, which had made it a genuine community platform, in order to push users toward Google+. Reader is the rug-pull as *strategic abandonment of a working community tool that didn't serve the ad/growth engine*, and it durably damaged trust in building anything on a free Google product. [Documented history; analysis.]

The cumulative effect was cultural. The "**Killed by Google**" phenomenon (a community-maintained graveyard of discontinued Google products) became shorthand for a specific lesson: building your community or workflow on a free Google product is risky because the company shutters products at will. This is the rug-pull as *abandonment* rather than extraction, and it taught users a durable distrust. [Documented history; analysis.]

### Facebook: from social utility to engagement machine to "pay to not be the product" (2004–2026)

Facebook is Doctorow's own worked example of enshittification, and the arc is well documented: an early phase good to users, a middle phase optimized for advertisers and engagement, and a mature phase extracting from everyone. Doctorow's three-stage model uses Facebook as its case study: first good to users, then abusing users to benefit business customers, then clawing back value for shareholders by drowning followed-account content in ads and pay-to-boost posts. [Documented history; analysis.]

The canonical privacy controversy is **Cambridge Analytica** (revealed March 2018): data on tens of millions of Facebook users (commonly cited as about 87 million) was harvested via a personality-quiz app and its friend-graph permissions, then used for political profiling. It became the defining example of the surveillance-advertising model's costs. [Documented history; `[UNVERIFIED]` on the exact 87M figure, which is Facebook's own later estimate.]

The recent and most thesis-relevant turn is the **paid ad-free subscription**. After the EU's top court ruled Meta must obtain consent before showing personalized ads, Meta in October 2023 rolled out a paid ad-free option for Facebook and Instagram in Europe, around €10/month on web and roughly €13 on iOS/Android. Users who decline to pay are asked to consent to data processing for personalized advertising, a structure widely described as "pay or consent." Privacy advocates argued this means users now *pay and are still the product* in the free tier, and that the paid tier monetizes privacy itself. Privacy activist Max Schrems argued the issue is not the fee amount but the "pay or okay" approach, contending that even a small fee shifts the vast majority of users into clicking "yes" and so does not constitute freely given consent under GDPR. [Documented history.]

This is the precise complication the commissioner wanted surfaced: "if you're not paying, you're the product" no longer cleanly holds, because the modern move is to charge users *and* keep the data engine running. [Analysis.]

### Twitter/X: a public square bought and bent (2022–2026)

Elon Musk acquired Twitter in October 2022 for about $44 billion and took it private. The repurposing was rapid and deliberate: verification was converted from an identity-signal into a paid subscription (Twitter Blue / X Premium), the previously free API was enclosed behind expensive tiers that broke researchers and third-party clients, content-moderation staff and policies were cut, and the platform was reoriented around the owner's preferences for reach and narrative. The result was repeated migration waves to alternatives: **Mastodon** (federated, ActivityPub), **Bluesky** (built on the AT Protocol), and Meta's **Threads**. [Documented history; `[UNVERIFIED]` on precise migration-wave user numbers, which fluctuated and were reported inconsistently.]

Twitter/X is the cleanest modern case of *repurposing*: not (only) monetization, but the conversion of a quasi-public communications commons into an instrument aligned with a single owner's agenda. It is the strongest argument that concentrated ownership of communications infrastructure is itself the risk, independent of the business model. [Analysis.]

### Vine: the creator-economy rug-pull (2013–2017)

Vine, the six-second looping-video app, is a distinct mechanic worth including: a community killed not by extraction but by *disinvestment* in a culture that generated value without ad revenue. Twitter acquired Vine for about $30 million in October 2012, before it had even launched. It grew to nearly 200 million active users by 2015, then had uploads removed and was fully shut down by 2017. It went offline on January 17, 2017 because it never built a revenue model, lost its top creators to better-paying competitors, and was deprioritized by Twitter. The loss landed unevenly: it was widely mourned as a specific blow to Black creators and people of color who had found an audience on Vine they could not reach through traditional media, and Vine had also been used to document the 2014 Ferguson protests. Twitter kept an archive online for a time and offered downloads, but the *community* (the thing that made Vine matter) was shuttered regardless. Vine shows that a platform does not need to turn extractive to rug-pull a community; a parent company can simply decide the culture is not worth funding. [Documented history; analysis.]

### Reddit: volunteer labor, the API revolt, and the AI-data turn (2005–2026)

Reddit is the richest single case because it combines almost every mechanic at once: volunteer/community labor, an API enclosure that destroyed a third-party ecosystem, an IPO, and data-licensing-for-AI.

The 2023 API revolt is well documented. In May 2023 Reddit announced new API pricing that would have cost the popular third-party app Apollo about $20 million per year, an unsustainable amount for an independent developer, and Apollo announced it would shut down on June 30, 2023. Apollo's developer Christian Selig said the new pricing worked out to $12,000 per 50 million requests, and that Apollo had made about 7 billion requests in the prior month. The change forced multiple third-party clients (Apollo, Reddit Is Fun, others) to close. [Documented history.]

The community response and the CEO's reaction crystallized the dynamic. More than 300 subreddits, including very large communities, joined an indefinite blackout, taking themselves private in protest. CEO Steve Huffman dismissed the protest and, in an NBC interview, compared the volunteer moderators to the "**landed gentry**." Huffman argued that politicians answer to constituents and business owners to shareholders, but that on Reddit "the people who get there first get to stay there and pass it down to their descendants, and that is not democratic." This was a striking framing: the unpaid labor that built the platform's value, recast as an illegitimate aristocracy. [Documented history; analysis.]

Then the monetization arc completed. On the same day in February 2024 that Reddit filed for its IPO, it announced a content-licensing deal with Google reportedly worth about $60 million a year, giving Google access to Reddit's user-authored content for AI training; an OpenAI partnership followed, estimated at around $70 million a year. Reddit went on to list on the NYSE under the ticker RDDT. [Documented history.]

The full Reddit arc is the thesis in miniature: value created by volunteer moderators and posters, an enclosure that killed the third-party ecosystem built on the open API, a public listing that formalized the obligation to extract, and the sale of the community's collective writing as an AI training asset, with no payment to the people who wrote it. [Analysis.]

### Discord: the platform on the runway (2015–2026)

Discord, founded in 2015 by Jason Citron and Stanislav Vishnevskiy, grew into a primary home for online communities while deliberately avoiding the advertising model, monetizing instead through Nitro subscriptions and server boosts. Discord reached an estimated ~$561 million in revenue in 2025 with roughly 260 million monthly active users, generating around $2.16 per user annually, far below ad-driven peers. [Documented history.]

The thesis-relevant fact is the trajectory, not an IPO that has not happened. Reuters reported Discord confidentially filed for a U.S. IPO on January 6, 2026. As of May 2026 Discord was still not publicly traded. In April 2025 co-founder Jason Citron stepped down as CEO, remaining on the board, replaced by Humam Sakhnini, a former Activision Blizzard executive. [Documented history.]

The signal is the setup for the squeeze. A community platform with very low revenue-per-user, a subscription-first model, and an incoming public-markets obligation faces exactly the pressure the cycle predicts: the gap between its current ARPU and ad-driven peers is the "monetization hurdle" investors will expect it to close, which points toward ads or deeper data monetization that its community-first culture was built to avoid. Analysts frame the challenge as scaling advertising into a large business without alienating a community-first user base. Discord has not rug-pulled; the point is that the structural incentive to do so is being installed now. [Analysis; commissioner's thesis that the IPO is the obligation to extract.]

### Further afield: niche cases that add new angles

The big platforms show the cycle at scale. A handful of smaller or stranger cases sharpen specific mechanics the marquee examples blur together.

**Something Awful: the paid model that resisted enshittification but still declined.** The comedy forum Something Awful is an important *counter*-case because it broke the "free, then extract" pattern from the start. After early advertisers failed to pay, founder Richard Kyanka began charging a one-time $10 fee for a forum account in 2001, and the site has drawn continuous income from account fees, forum upgrades, and merchandise ever since (an account still cost $10 as of 2025). By charging users directly, SA never needed to convert its community into an ad product, and it avoided the engagement-optimization spiral. Yet it still declined, because culture migrated to Twitter, Reddit, and 4chan. As one participant put it in an oral history, the community was not so much killed as relocated: it became easier to congregate elsewhere, and people brought the community with them. The lesson cuts against romanticizing any single fix: a sustainable, user-funded model defeats the *extraction* failure mode but not the *network-migration* one. Paying users still leave when the network moves. [Documented history; analysis.]

**City of Heroes: the MMO as community, IP custody, and a rare happy ending.** When a game is also a community, shutdown erases not just chat logs but a persistent shared world. In 2012 NCSoft closed Paragon Studios and shut down the superhero MMO City of Heroes, taking eight years of characters and shared history offline despite a fan campaign to save it. The interesting part is what followed, and what it reveals about custody. Fans kept a functioning private server secret for years specifically because NCSoft owned the code and IP and had issued cease-and-desist orders against a similar revival of another shuttered NCSoft game. The community's ability to preserve its own world was hostage to copyright, not just to servers. The resolution is the rare positive one: in January 2024 NCSoft granted the fan-run Homecoming project an official license to operate a City of Heroes server and keep developing the game, free and donation-funded, while NCSoft retained the IP. This case adds two things the others lack: the persistent-world stakes of community death, and a demonstration that the rights-holder *can* choose to hand custody back, though only at its discretion and a decade late. [Documented history; analysis.]

These two, alongside the LiveJournal/Dreamwidth and Yahoo/Upcoming stories already covered, point at a pattern in the *recoveries*: every community that got its world back did so through volunteer preservation plus either a forkable codebase (Dreamwidth) or an eventual act of grace by the owner (City of Heroes, Upcoming). None recovered through a right they held at the outset. That is the gap the synthesis has to close. [Analysis; commissioner's thesis.]

**Confirming variants (same mechanics, different decade).** A few further cases recur the patterns above closely enough not to need full treatment, but they show the pattern's consistency. NCSoft shut down its MMO Tabula Rasa in 2009 and issued a cease-and-desist against a fan revival, the precedent that made the City of Heroes preservationists keep their server secret. FanFiction.net ran its own content purges (notably a 2002 ban on explicit work, and later sweeps) that, like LiveJournal's Strikethrough, taught fandom not to trust a single commercial host, feeding directly into the creation of AO3. And in 2013 Amazon launched **Kindle Worlds**, a commercial fanfiction platform that, like the earlier FanLib, sought to profit from fanworks under terms many fans rejected; it shut down in 2018. The repetition across gaming, fandom, and commerce is the point: the same machinery operates regardless of the vertical. [Documented history; `[UNVERIFIED]` on the precise FanFiction.net purge dates and Kindle Worlds terms, which are well attested in general but not re-verified here.]

---

## Part II: The recurring mechanics

The chronology is not a list of unrelated failures. The same machinery recurs.

**1. Network effects: collectively created, privately captured.** The value of a social platform is made by its users (their posts, their moderation, their social graph, their archives) but is owned by the company. This asymmetry is the precondition for everything that follows: the people who create the value have no claim on it. Reddit's volunteer moderators are the sharpest illustration, their unpaid labor later licensed to AI companies for tens of millions. [Analysis.]

**2. Enshittification: the behavioral pattern.** Doctorow's formulation, which he popularized in a November 2022 post and which the American Dialect Society named its 2023 Word of the Year: "first, they are good to their users; then they abuse their users to make things better for their business customers; finally, they abuse those business customers to claw back all the value for themselves. Then, they die." He has also called it "platform decay," and ties it to the ease of reallocating value on a platform combined with the dynamics of a two-sided market where the platform holds each side hostage to the other. This is the best available *description* of the cycle. [Documented history; analysis: it describes behavior, the next two mechanics explain why the behavior is near-inevitable.]

**3. The VC growth imperative: why "free and good" is a funded phase.** Venture capital is not patient capital; it requires a return, which requires an exit (acquisition or IPO). The "good to users" phase is, structurally, the investor-subsidized customer-acquisition phase. The IPO or sale is the moment the subsidy must convert into extraction. This is why the Reddit IPO and the Discord filing matter as *signals*: going public formalizes a fiduciary obligation to grow profit, which on a mature platform means extracting more from users and partners. [Analysis; commissioner's thesis, here stated as the core claim: the cycle is a capital-structure phenomenon before it is a technology or values one.]

**4. Lock-in and the loss of exit.** Switching costs are what let extraction happen without losing the users. The social graph is captured (your friends are here), identity is non-portable (your handle and history do not travel), and data is not yours to take. Doctorow's own emphasis: platforms are deliberately designed to be "hard to leave," so moving an entire extended social circle to a new platform is very, very hard. Lock-in is the leverage; without it, the moment a platform turned on its users they would simply walk. [Analysis.]

**5. Data and content custody: the platform-death failure mode.** When the platform dies or decides to delete, the community's accumulated memory dies with it, because the users never held custody. GeoCities (2009) and Yahoo Groups (2019–2020) are the pure cases, and MySpace's music loss (2019) the accidental one. This is distinct from exploitation: it is erasure, and it is arguably the most irreversible harm in the entire pattern. [Analysis; commissioner's thesis that content custody is a survival question.]

**6. "If you're not paying, you're the product" and its modern complication.** The adage long captured the surveillance-advertising bargain. The Meta "pay or consent" model breaks it: in the free tier you are still the product, and the paid tier sells you the privilege of not being tracked, so the data engine is monetized coming and going. The lesson is that paying for a service inside a concentrated, extraction-driven ownership structure does not buy you out of the cycle; it just adds a revenue line. [Analysis.]

---

## Part III: The antidotes, and why most failed

The counter-tradition is real and partly successful. It is also full of honest failures, and romanticizing it would undercut the thesis rather than support it.

### Open protocols and federation

The argument is "you can't rug-pull a protocol." Email, Usenet, IRC, and now **ActivityPub** (Mastodon and the wider Fediverse), the **AT Protocol** (Bluesky), and **Matrix** (chat) decentralize control so no single owner can enclose or delete the network. Where it has held up: email and the web are still here decades on, precisely because no one owns them. Where it has failed:

- **UX and onboarding.** Mastodon's instance-selection and federation model is confusing to mainstream users compared with a single-app signup. Federated systems historically trade convenience for resilience.

- **Sustainability.** Instances are often run by volunteers; funding and moderation burdens cause instances to close, taking their local communities with them, which reproduces the platform-death problem at smaller scale.

- **Fragmentation.** Usenet's own decline shows a no-owner system can still rot through spam and abuse. Protocols resist *capture* but not *neglect*.

Bluesky is an instructive hybrid: it is a VC-funded company building on an open protocol, which means the protocol provides an exit option (you could in principle leave with your identity) even though the flagship app is corporate. Whether that exit is real in practice, at scale, is still unproven. `[UNVERIFIED: Bluesky's account-portability guarantees are designed-in but their real-world, at-scale exercise is not yet well documented.]` [Analysis.]

### Cooperative and community-ownership models

**Platform cooperativism**, a movement associated with Trebor Scholz (who helped popularize the term around 2014), proposes platforms owned and governed by their workers and users, modeled on cooperative and credit-union principles. The appeal is direct alignment: the people who create the value own it. The struggles are equally direct: raising capital without VC is hard, scaling governance is hard, and co-ops have repeatedly lost on UX and growth speed to well-funded competitors. Two concrete cases bracket the range of outcomes.

**Stocksy United is the clearest co-op success, and its origin is on-thesis.** Stocksy was founded in 2013 by Brianna Wettlaufer and Bruce Livingstone, formerly of iStockphoto, which had been bought by Getty Images; they built an artist-owned cooperative paying photographers 50–75% of each license, far above industry norms. The structure is deliberately rug-pull-proof. Because it is a co-op there is no equity for a venture capitalist to buy, share value does not increase, and the co-op principles prevent outside financial control; it was funded by a $1 million founder loan repaid within three years. The inability to sell the company without member consensus is built into the model by design, and Stocksy has paid out tens of millions in royalties to its artist-members. The founders had watched their previous creation get absorbed by a giant, and built the next one so that could never happen. It works, but note the scale: it is a curated marketplace capped near 1,000 members, not a mass social network. [Documented history; analysis; commissioner's thesis demonstrated.]

**Resonate, the music-streaming co-op, is the cautionary version.** Founded as a multi-stakeholder cooperative in Ireland in 2016, Resonate is owned by its artists, listeners, and workers, and uses a "stream2own" model where the price of a track doubles with each play until the listener owns it after nine plays. The values are impeccable and the model is genuinely novel, but it has struggled to scale, sustain funding, and stabilize leadership, remaining small (a few thousand artists) with repeated relaunches and stretches of stalled development. It is the honest counterpoint to Stocksy: a co-op with the right governance and the wrong luck on funding and execution, which is exactly the failure mode the thesis warns about. Governance alone does not save you if you cannot fund the experience. `[UNVERIFIED: the specific details of Resonate's funding crises and leadership turnover are widely discussed in community channels but I did not confirm them to a single authoritative source; the small scale and repeated relaunches are well attested.]` [Documented history; analysis.]

The broader pattern holds: working co-ops exist in specific niches (Stocksy is the standout), but no cooperative has become a mainstream social-network-scale success, and the difference between Stocksy and Resonate is mostly funding and execution, not values. [Analysis.]

### Nonprofit and foundation governance: the rare durable successes

This is where the durable exceptions live, and studying *why* they endured is the payoff.

- **Wikimedia / Wikipedia.** Wikipedia is the standout: a top-tier global site that never enshittified. The structural reasons are specific. The content is openly licensed (so it cannot be enclosed; anyone can fork the entire encyclopedia), it is governed by a nonprofit foundation rather than owned by shareholders, it carries no advertising, and it is funded by donations. There is no exit event to extract for, because there is no equity to sell. The community can, in principle, leave with everything, which removes the leverage for any future rug-pull. [Documented history; analysis: this is the model that proves the thesis.]

- **Signal.** Signal deliberately rejected the VC path. Co-founder Moxie Marlinspike wrote that Signal never took VC funding or sought investment because putting profit first would be incompatible with a sustainable, user-first project. In February 2018 the Signal Foundation launched as a 501(c)(3) nonprofit with a $50 million contribution from WhatsApp co-founder Brian Acton, intended to free Signal from "the inherent limitations of a for-profit company." Signal is not owned by Microsoft or any company, a confusion worth correcting directly (the Microsoft association belongs to Skype, which Microsoft bought in 2011 and retired in 2025). The Signal Foundation, a nonprofit, wholly owns Signal Messenger. Its current president, Meredith Whittaker, states the structural logic bluntly: privacy is not safe under shareholder-driven corporations, so Signal is structured as a nonprofit with no equity and no board demanding profit growth, which means it is not possible to sell Signal off "even if I decided to become evil."

  One important nuance for the thesis: Signal is *architecturally centralized*. It runs its own servers and has historically resisted federation and third-party clients, so its protection is **governance, not decentralization**. The servers are central, but the entity controlling them legally cannot be sold or taken public. This is a different defense from the open-protocol crowd's (decentralize so the servers don't matter), and the contrast is instructive: you can be centralized-and-safe if the ownership structure removes the extraction motive. The caveat is funding. Signal's 2024 expenses (about $38 million) exceeded its revenue (about $29 million), and it relies on donations plus the depleting Acton funds, so its long-term sustainability is unusually structured but not solved. The endowment's irony is also pointed: it came from Acton's wealth from selling WhatsApp to Facebook for about $19 billion in 2014, so a successful extraction funded the anti-extraction model. [Documented history; analysis.]

- **Mozilla.** A nonprofit-governed organization (the Mozilla Foundation owns the Mozilla Corporation) that has kept Firefox alive as an independent browser. It is a more cautionary success: its dependence on a search-default deal with Google for the large majority of its revenue shows that nonprofit governance does not by itself solve the funding problem, and can create a different dependency. `[UNVERIFIED: the precise share of Mozilla revenue from the Google deal varies by year; the dependency is well documented in general terms.]` [Documented history; analysis.]

- **Archive of Our Own (AO3) / Organization for Transformative Works.** This is the most thesis-perfect case of all, because it was designed *as a direct response to a rug-pull*, by the very community that got burned. In 2007 fan-fiction writers faced two threats at once: FanLib, a commercially owned for-profit fanfiction archive widely perceived as outsiders trying to profit from fans' work, and "Strikethrough," LiveJournal's May 29, 2007 mass suspension and deletion of journals and communities without warning, which swept up legitimate fan and even abuse-survivor communities. The response was structural, not reactive. The writer astolat proposed a fan-controlled archive (no ads, no content restrictions, a commitment to fanworks as fair use), and the deletions drove home that it was not safe to rely on commercial entities to preserve fan culture, nor on any platform "subject to a single person's whims." The result, the OTW and AO3, is now a top-tier site by any measure. AO3 is a nonprofit, non-commercial archive run on open-source software the OTW developed, sustained entirely by user donations, hosting tens of millions of works for over 8 million users, ad-free by principle, and it won a 2019 Hugo Award. The lesson is exact and reinforces the LiveJournal case above: the community that lived through deletion rebuilt on infrastructure it owned, governed by a nonprofit that cannot be sold, funded by its own users. It is Wikimedia's model proven a second time, in a very different domain. [Documented history; analysis; commissioner's thesis demonstrated end to end.]

The common thread among the survivors is not better software. It is that each solved funding in a way that *removed the extraction imperative*: open license plus donations (Wikimedia), endowment plus donations and a foundation that cannot be sold (Signal), volunteer labor and donations on community-owned open-source infrastructure (AO3). [Analysis; commissioner's thesis.]

### Legal and governance instruments

- **Data portability law.** The EU's GDPR (2018) created a right to data portability, and the Digital Markets Act (DMA, in force 2024) adds interoperability and anti-self-preferencing mandates for "gatekeepers." These attack lock-in directly by trying to make exit a legal right. The Meta "pay or consent" fight shows both the power and the limits: regulation forced Meta to offer a choice, but Meta engineered the choice to preserve the data business, and the adequacy of "pay or okay" consent is still contested. [Documented history.]

- **Licensing and charters.** Open content licenses (Wikipedia) and open-source software licenses make enclosure structurally impossible by guaranteeing the right to fork. Nonprofit charters and trust structures can make an organization legally unsellable. These are the instruments that, combined with portability, give the governance half of the thesis its teeth. [Analysis.]

- **Toothless versions.** Voluntary "data download" tools (the Yahoo Groups "GetMyData" process, platform-provided archive exports) are portability in name only when the data lands in a non-interoperable format that cannot rebuild the community elsewhere. During the Yahoo Groups wind-down, Verizon even banned volunteer archivists and cited terms of service prohibiting third-party extraction tools, while offering an individual self-service download. Portability that you cannot actually act on at community scale is not exit. [Documented history; analysis.]

### Honest failures

The alternatives let people down too. Usenet rotted despite being un-ownable. Federated instances close and take communities with them. Co-ops have largely failed to scale. Countless FLOSS and self-hosted communities died of maintainer burnout. The point is not that community-owned models are safe; it is that the *specific* survivors solved a *specific* problem (funding without an extraction imperative) that the failures did not. [Analysis.]

---

## Part IV: Synthesis: how to break the cycle

### The structural diagnosis

Four causes matter most, and they compound:

1. **The VC extraction imperative.** Growth capital requires an exit; an exit requires extraction. This is the engine.

2. **Lock-in via captured social graph and non-portable identity.** This is the leverage that lets extraction proceed without losing users.

3. **Un-owned data and content.** This is what makes both monetization-of-the-user and outright deletion possible, and it is the cause of the most irreversible harm.

4. **Concentrated ownership of the communications layer.** This is what makes repurposing (Twitter/X) possible: one owner can bend the commons to an agenda.

A durable alternative must *structurally prevent* each of these, not merely promise to behave. [Analysis; commissioner's thesis.]

### Coupling governance and UX (the core thesis)

The community/FLOSS side historically delivered the governance half (open protocols, co-ops, nonprofits) and failed on the experience half (onboarding, reliability, brand stability, single-app simplicity). The VC side delivered the experience half and built the extraction trap. The thesis is that you need *both*, and the survivors show what "both" looks like:

- **Governance half:** an ownership structure that legally cannot be sold out (nonprofit, foundation, trust, or genuine cooperative), open protocol and/or open content license so the community can always fork and leave, and portable identity and data so exit is real.

- **Experience half:** the polish, reliability, and coherent brand that make a normal person actually use it. Signal's lesson is that a nonprofit *can* ship a mainstream-quality app; Wikipedia's lesson is that a nonprofit *can* run top-tier infrastructure. The failure mode of the alternatives was treating governance virtue as a substitute for usability rather than a constraint to design within.

Neither half alone works. Better UX inside an extractive ownership structure just delays the rug-pull (Discord's trajectory). Pristine governance with bad UX never reaches the scale where it matters (most co-ops, much of the Fediverse). [Commissioner's thesis, developed; analysis.]

### The portability/exit insight

Making "leave with your community and data intact" structurally real is what removes the leverage for a rug-pull. If identity is portable, data is owned by the user, and the protocol is open, then the moment a platform turns extractive, the community can walk, which means the platform *cannot* turn extractive without dying first. This is the deepest reason data custody is not a nicety: it is the mechanism that disarms the entire cycle, and it is also the defense against the deletion failure mode, because a community that holds its own data cannot be erased by a company's shutdown decision. [Analysis; commissioner's thesis.]

### The funding question (the crux)

This is the hard center, and the history is brutal about it. Every durable survivor solved funding *unusually*: Wikimedia by donations atop an openly licensed, ad-free, unsellable structure; Signal by a large endowment plus donations inside a foundation that cannot be sold. The available non-VC models (endowment, nonprofit donations, cooperative dues, public/grant funding, modest sustainable subscription revenue) each have failure modes, and none has yet produced a mainstream *social network* at Facebook scale. Mozilla shows that nonprofit status without an independent revenue base just relocates the dependency.

The uncomfortable conclusion: the funding-and-governance commitment must be made *at inception*, because the history shows you cannot retrofit it after taking growth capital. Once equity is sold to investors who require a return, the extraction imperative is installed, and no later governance bolt-on removes it. Retrofitting governance after growth capital is precisely the thing that fails. [Analysis; commissioner's thesis, stated as the central practical warning.]

### The realistic verdict

This is genuinely hard, and honesty requires saying so. The survivors are rare, and they had things in common: they avoided the VC extraction imperative from the start, they made their content or protocol forkable so exit was real, they put ownership in a structure that could not be sold, and they still managed to deliver enough usability to reach scale. A new attempt would need, from day one: a non-extractive funding model and an unsellable ownership structure (the governance half), portable identity and user-owned data on an open protocol (the exit/leverage half), and a genuine investment in experience and brand stability (the half the alternatives skipped).

### Prioritized take

**The 3–4 structural causes that matter most:**

1. The VC extraction imperative (the engine; an exit demands extraction).

2. Non-portable identity plus a captured social graph (the lock-in that supplies the leverage).

3. Un-owned user data and content (enables both monetization and irreversible deletion).

4. Concentrated, sellable ownership of the communications layer (enables repurposing).

**The 2–3 antidote combinations with the best track record:**

1. Open license/protocol + nonprofit-or-foundation governance + donation/endowment funding (Wikimedia, Signal). This is the only combination that has actually produced durable, non-enshittifying, at-scale-or-near-scale services.

2. Open protocol + real, exercised account portability (the Fediverse/AT-Protocol aspiration), which disarms lock-in *if* the exit is genuinely usable at community scale, currently the unproven part.

3. Portability/interoperability law (GDPR/DMA) as a backstop that raises the cost of lock-in, useful only when paired with formats and protocols that make the exported data actually rebuildable elsewhere.

**The single most important structural commitment at inception:**

Put the platform's ownership in a structure that legally cannot be sold or taken public, and fund it without VC growth capital, *before* taking any money that carries an extraction imperative. Everything else (portability, open protocol, good UX) reduces the *leverage* for a rug-pull, but only the funding-and-governance commitment removes the *motive*, and it is the one thing the history says you cannot add later.

---

## Sources and verification notes

Verified to current/strong secondary sources during research: Discord's pre-IPO status and January 2026 confidential filing; the 2025 revenue/MAU figures; the Reddit 2023 API revolt (Apollo's ~$20M figure, the $12,000/50M-requests pricing, the 7 billion requests, the June 30 2023 shutdown), Huffman's "landed gentry" remark, the 300+ subreddit blackout, the ~$60M Google deal announced alongside the February 2024 IPO filing and the ~$70M OpenAI deal, and the NYSE RDDT listing; Yahoo Groups' 2019 content-deletion announcement and December 15 2020 shutdown, and Archive Team's ~1.5M groups / ~2.1B messages / ~1.8B archived figures; Yahoo Chat's June 2005 advertiser-driven shutdown, the October 2005 AG agreement and age restriction, the December 14 2012 chat-room discontinuation, the ~75%-bots 2007 estimate, and the 2018/2019 Messenger and Together shutdowns; the Yahoo acquisition cluster (Flickr ~$35M and Delicious ~$15–20M in 2005, the acquire-neglect-abandon pattern, the December 2010 "sunset" slide, Upcoming's ~11-day-notice closure and Archive Team rescue, and its later relaunch by Baio); Google Reader's July 1 2013 shutdown, the 46,000+ signature petition, the stated reasons, and the 2011 stripping of its social features toward Google+; LiveJournal's 2007 SUP sale, the December 2016 server move to Russia, the April 2017 ToS change and "media outlet" threshold, the monitoring critique, and the Dreamwidth fork/import exit; Vine's ~$30M 2012 acquisition, ~200M users, January 17 2017 shutdown and the reasons, and the disproportionate impact on Black creators; Something Awful's 2001 introduction of the $10 account fee and its ongoing fee-based model, plus the migration-driven decline; City of Heroes' 2012 NCSoft shutdown, the secret fan server kept hidden for fear of cease-and-desist, and the January 2024 official Homecoming license; the AO3/OTW origin (the 2007 FanLib backlash and the May 29 2007 LiveJournal "Strikethrough" deletions, astolat's archive proposal, and AO3's nonprofit/open-source/donation-funded model, 8M+ users, and 2019 Hugo Award); Digg's August 2010 v4 redesign, the removal of the bury feature and publisher auto-submission, the August 30 2010 "Quit Digg Day" revolt, the ~25% traffic drop and Reddit's 230% 2010 growth; the SmugMug acquisition of Flickr in April 2018, MacAskill's anti-data-mining framing, the November 2018 end of the free 1TB tier and 1,000-photo cap, and the December 2019 plea for more paying users; Stocksy United's 2013 founding by ex-iStockphoto figures, the 50–75% royalty rate, the no-equity/cannot-be-sold co-op structure and $1M founder loan; Resonate's 2016 founding as a multi-stakeholder co-op and its stream2own model; GeoCities' October 26 2009 shutdown and the Archive Team rescue (~900GB torrent); Doctorow's enshittification definition, the November 2022 origin and the 2023 American Dialect Society Word of the Year; Meta's October 2023 EU paid ad-free launch, pricing, the "pay or consent" structure, and Max Schrems' critique; and Signal's nonprofit structure and full ownership of Signal Messenger, the $50M Acton contribution, Marlinspike's no-VC statement, Whittaker's "even if I decided to become evil" framing, and the 2024 expenses-exceed-revenue figures.

Treated as stable historical record but not independently re-verified in this pass (flagged `[UNVERIFIED]` inline where load-bearing): GeoCities' 38M-pages/3M-users peak and the $3.6B Yahoo acquisition price; MySpace's ~$580M News Corp purchase, ~$35M resale, and the 50M-songs/12-years loss; the Cambridge Analytica ~87M figure; Google Buzz FTC settlement specifics; MySpace/Friendster decline mechanics; Twitter migration-wave user counts; the exact Andy Baio "horrible mistake" quote wording; Resonate's funding-crisis and leadership specifics; the FanFiction.net purge dates, Tabula Rasa C&D, and Kindle Worlds terms; Bluesky's at-scale portability; and Mozilla's exact Google-revenue share.
