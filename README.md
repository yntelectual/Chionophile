# Chionophile
Demonstrator on how to achieve energy efficiency improvements  in software development

## Requirements
- Windows OS with powershell
- git bash
- wsl
- Docker Desktop
- python3

## Optional requirements
- node
- npm
- java (jdk 11)
- maven
- pip
- rust
- cargo
- subo (wsl)

## Aim
Compare programming languages performance and energy consuption in real world usage, implementing a sample application called [Jump The Queue](https://github.com/devonfw/jump-the-queue).

Currently, the analyzed languages are:

- Python (Django REST)
- NodeJS (Nest.js)
- Java (Springboot)
- C# (ASP.NET)
- Rust (Actix4)

The applications are completely interchangeable, as they share the same behaviour in all of the endpoints.

Additionally, a *wasm* compiled rust version was implemented with the *subo* tool.

## Benchmarks
Two benchmarks are executed for different purposes:
1. **Energy consumption benchmark**: for 5 minutes the application will receive 14 requests per second. Maintaining this request rate will show how the different backends perform under the same load.
2. **Performance benchmark**: backends will receive as many requests as possible. This will show the maximum load capacity for each language.


## Test flow

The testing flow is composed by the following statements:

- Get the idle energy consumption for 1 minute
- For each language:
  - Launch the Postgres database and the backend with docker-compose.
  - Wait for 2 min.
  - Start Benchmarks sequentially
  - When the benchmarks are completed, the backend and Postgres containers are killed.
- Remove unnecessary data and rows from the generated CSV and HTML, and wrap the relevant information in HTML files

## Results

The execution of the automated test script will generate a report in the results folder. Some results are currently placed in this folder, both in PDF and HTML. We encourage you to download the HTML as it has dynamic content not visible in the PDF counterpart, and will ease the result's observation. Here is a summary of the results (using docker images):

### Benchmark 1 
|                                         |  Rust      |  Java   |  Wasm    |  NodeJS   |  Python  |  .NET (without docker)   |
|:---------------------------------------:|:----------:|:-------:|:--------:|:---------:|:--------:|:------------------------:|
| Consumption B1 (W) (idle excluded)      | **1.55**   | 2.00    | 2.03     | 2.17      | 3.68     |   4.57                   | 
| Consumption % (idle excluded)           | -          | +29%    | +31%     | +40%      | +137%    |   195%                   |

### Benchmark 2
|                                     |  Rust      |  .NET (without docker)    |  Java   |  NodeJS  |  Wasm    | Python |
|:-----------------------------------:|:----------:|:-------------------------:|:-------:|:--------:|:--------:|:------:|
| Total request B2                    | **1125535**| 653153                    | 436546  | 260920   |  158236  | 136474 |
| Request / second                    | **3751**   | 2177                      | 1455    | 869      |  527     | 454    |
| Consumption B2 (W) (idle excluded)  | 15.22      | 16.67                     | 14.78   | 14.86    |  10.08   | 14.88  |
| Performance %  (Total request)      | -          | -72%                      | -158%   | -331%    | -611%    | -625%  |


## Bibliography

This project has taken multiple studies and articles as a basis for the tools, hypothesis and methods used.

* [Ranking Programming Languages by Energy Efficiency](https://haslab.github.io/SAFER/scp21.pdf)
* [What are Your Programming Language's Energy-Delay Implications?](https://ieeexplore.ieee.org/document/8595213)
* [Analyzing Programming Languages’ Energy Consumption: An Empirical Study](https://stefanos1316.github.io/my_curriculum_vitae/GKS17.pdf)
* [AWS: Sustainability with Rust](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/)
* [Why Discord is Switching from Go to Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
* [How Programming Languages and Paradigms Affect Performance and Energy in Multithreaded Applications](https://ieeexplore.ieee.org/stamp/stamp.jsptp=&arnumber=7828287)
* [On Reducing the Energy Consumption of Software: From Hurdles to Requirements](https://hal.inria.fr/hal-02892900/document)
