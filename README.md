# microservice-pattern

## Concept

This crate advocates the use of **in-process microservice pattern**.

The typical modern system is consist of bunch of gRPC micro services that communicates each other. The gRPC microservice is a process whose internal is consist of logics and data structures. 

To develop the internal, in-process microservices is a very helping layer because you can modularize the internal in the same way the gRPC services does in the inter-process layer.


```mermaid
graph LR
  SYSTEM(System)
  GRPC(gRPC Microservice)
  subgraph In Process
    MICRO(Microservice)
    DATA(Data Structure)
	LOGIC(Logic)
  end
  SYSTEM -->|1..| GRPC -->|1..| MICRO
  MICRO -->|1..| DATA
  MICRO -->|1..| LOGIC
```

## Ticking

A processing of in-process micro services is triggered by either
gRPC request or internal tick.

To make a internal tick you can use tools in tokio::time or 
[tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler)
for more flexible control of the ticks.

```mermaid
graph LR
  S(Service)
  T(Ticker)
  T -->|call| S
```

## Dependency loop

Dependency loops are not allowed in this library because it is a vicious way of designing micro services but it is sometimes inevitable. To work around, queue can be used to invert the dependency. You may use actor framework to implement this.

```mermaid
graph LR
  S1(Service1)
  S2(Service2)
  S1 -->|1. call_A| S2
  S2 -->|2. call_B| S1
```

==

```mermaid
graph LR
  S1(Service1)
  S2(Service2)
  EX(Executor)
  Q(Queue)
  S1 -->|1. queue task| Q
  Q -->|2. dequeue task| EX
  EX -->|3. call_A| S2
  EX -.->|dependency| Q
  S2 -->|4. call_B| S1
```

This is quite similar to sidecar pattern.

```mermaid
graph LR
  S1(Service1)
  S2(Service2)
  S3(Queue Service)
  S1 -->|1. queue task| S3
  S2 -->|2. find new task| S3
  S2 -->|3. call| S1
```