# ddai

What is `ddai`? It's an abbreviation of `Domain Driven (powered by) AI`. It's a tool to help software engineers, especially software architects, manage their specific business domain knowledge and interpret it's knowledge into software components by following the `DDD (Domain Driven Design)` approach.

## Overview

`Domain Driven Design (DDD)` is a software development approach that focuses on modeling software to match the business domain and its complexity. At its core, DDD emphasizes close collaboration between technical teams and domain experts to create software that truly reflects the business reality.

## Core Philosophy

DDD operates on the principle that the most important part of software development is understanding the business domain - the sphere of knowledge and activity around which the business application revolves.

Rather than starting with technical concerns, DDD begins with deep domain understanding and lets that drive the technical implementation.

## Concepts

### Strategic Concepts

- **Ubiquitous Language**
	- Forms the foundation of DDD. This is a common vocabulary shared between developers and domain experts, used consistently in code, documentation, and conversations. The language evolves naturally as understanding deepens
- **Bounded Contexts**
	- Explicit boundaries within which a particular domain model applies. Different parts of a large system may have different models of the same concept - for example, a "Customer" in the sales context might be quite different from a "Customer" in the support context. Each bounded context maintains its own model and ubiquitous language
- **Context Mapping**
	- Describes the relationships between different bounded contexts and how they integrate. This includes patterns like Shared Kernel, Customer-Supplier, and Anti-Corruption Layer.

### Tactical Concepts

- **Entities**
	- Objects with distinct identity that persists over time. Two entities with the same attributes are still different if they have different identities
- **Value Objects**
	- An immutable objects is defined by its attributes rather than identity. Examples include money amounts, dates, or coordinates
- **Aggregates**
	- Clusters of related entities and value objects treated as a single unit for data changes. Each aggregate has a root entity that controls access to the aggregate's internals
- **Domain Services**
	- It contains domain logic that doesn't naturally belong to any entity or value object.
- **Repositories**
	- Provide an abstraction for accessing aggregates, typically hiding persistence details.
- **Domain Events**
	- Represent something significant that happened in the domain

## DDD Process

DDD typically involves extensive knowledge crunching sessions where developers work closely with domain experts to understand the business. This collaborative modeling process helps identify the core domain (what makes the business unique), supporting subdomains (necessary but not differentiating), and generic subdomains (common solutions that can be bought or outsourced).

The approach emphasizes iterative refinement of both understanding and implementation, recognizing that domain knowledge evolves as the team learns more about the business problem they're solving.

DDD is particularly valuable for complex domains where the business logic is intricate and constantly evolving, though it may be overkill for simple CRUD applications or highly technical systems with minimal business complexity.

## Problems

DDD is indeed highly subjective and interpretation-dependent. The same business domain can be modeled completely differently by different engineers, and there's often no clear "right" answer.

This subjectivity manifests in several ways:

- **Bounded Context Boundaries**
    Where one engineer sees a single context, another might split it into three. There's no mathematical formula for determining these boundaries
- **Aggregate Design**
	The decision of what to include in an aggregate and where to set consistency boundaries is heavily influenced by the engineer's experience and perspective
- **Entity vs Value Object**
	Decisions often come down to individual judgment calls about what constitutes "identity" in the business context

## Solutions

There are some solutions to solve the subjectivity's issue

- **Document Decision Rationale**
	Keep records of why you made specific modeling decisions. This helps with consistency and knowledge transfer
- **Event Storming and Collaborative Modeling**
	Use structured workshops to reduce individual bias and get multiple perspectives on domain boundaries
- **Embrace Evolutionary Design**
	Accept that your initial model will be wrong and build in the ability to refactor bounded contexts and aggregates as understanding improves
- **Focus on Communication**
	Sometimes the "wrong" model that everyone understands is better than the "right" model that only one person grasps

## The DDAI Approaches

### Knowledge Base Management

The main purpose and objective of implementing `DDD` is to model specific and complex business domain knowledge into managed and working software. The engineering process to interpret specific business requirements it will depends on the engineer's knowledge.

Knowledges need to maintains:

- The business domain knowledges
- The DDD knowledge
- The software architecture knowledges

### Document Management

The `ddai` will maintain and manage two important documents:

- The business documents
- The technical software documents

All available documents will implement versioning by following the `SemVer (Semantic Versioning)` standard.

> `ddai` will only support `Markdown` content format

### LLM Powered Business Interpretations

By gathering all possible knowledge from both business and technical software engineering principles, `ddai` will empower *LLM* to interpret business domain knowledge and combine it with *RAG* technology, it will produce a "standard" DDD tactical design patterns.