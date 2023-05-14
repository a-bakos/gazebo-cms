# WIP Rust CMS
## A thought experiment project inspired by WordPress

This is a project for myself to practice and grow my Rust skills. The main idea behind this project is to explore and
re-create some parts of WordPress using Rust in order to pick up new things, solidify what I've learnt already, 
understand and commit to muscle memory.

**Why WordPress?**

Because I love WordPress and I know it like the back of my hand. It's that simple.

**Aims and limitations:**

Eventually, I want a prototype for a basic CMS flow. _However..._

Certainly, re-implementing a WordPress-like CMS is a non-trivial and rather enormous undertaking, something I certainly 
wouldn't be able to complete alone within a sensible timeframe.

But, at this point, it's not my goal to fully achieve that. I'm also not saying that I would not want it to be my goal in the future. 

Right now, my aim is to learn about Rust, practice it regularly, and in varied ways, maybe even to show it to some people for feedback. 
And for that purpose an extensive CMS (aka WP, which nowadays is more like a framework in my opinion) is a great subject.

Cherrypicking certain parts for re-implementation is a great method for me to keep this project and the learning experience interesting. 
It is a great exercise to think critically about data structures and modelling, databases and communication, content creation and user management, etc.

Even a small module, like permalink-generation has a lot of hidden knowledge, something that has already taught me a great deal and not just about Rust per se.

I'm not concerned with asynchronous approaches right now. I do want to look at that soon, though. The same applies for security and authentication.

Database-wise, I'm starting with one of the easiest possible ways, using a simple CSV format to store post and user data.
Later, I will look into real solutions, like PostgreSQL or SurrealDB. 

**What if...**

Who knows/wishful thinking/etc., and it's a big _if_ at this point, but if I can reach escape velocity with this prototyping exercise, there's always the potential it grows into a real product...
