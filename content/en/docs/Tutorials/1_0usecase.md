---
title: "1.0 Use Case"
date: 2022-03-17
weight: 4
description: >
  A brief use case of secure aggregation
---

People fill out online surveys from time to time, and a majority of the surveys claim that they will guarantee **anonymities** to their participants. People also are confident that privacy will not be invaded, since they don't write their names or addresses in the survey. However, their responses are sent to the organizers **intact**, without any processing, along with their IP addresses. 

In this tutorial, we propose a privacy-preserving way for servers(survey organizers) and clients(participants) to transfer data to each other.  We ensure that the survey organizer receive the overall results of the survey, but not know about each individual questionnaire. To accomplish this goal, we need to make use of **Secure Aggregation protocols**.

> **Secure Aggregation protocols** allow a collection of mutually distrust parties, each holding a private value, to collaboratively compute the sum of those values without revealing the values themselves. ([Source](https://research.google/pubs/pub45808/))

However, secure aggregation protocols need all clients(participants) to be online when the server(survey organizer) is collecting the survey answers. With this condition, we propose the following toy scenario for the tutorial.

University wants to estimate students' pressure by collecting their average sleep time per day. All participants receive the survey on Day 1, and have to fill out the survey with their sleeping time before Day 2. On Day 2, all participants need to tell University their IP addresses. On Day 3, University will collect the results from participants and all participants need to be online this day. University will then use secure aggregation to collect the sum of all participants' sleeping time without sacrificing the privacy of any participant. University can then calculate the average sleeping time by dividing with the participant count.
