# ferrokrait - /ˈfɛroʊkreɪt/
An ECS-like python library for game development written in rust. (WIP + Pain)
## Concept
ferrokrait provides a structure similar to an Entity Component System (ECS) called a `NodeSystem`. A `NodeSystem` can contain any number of individual `Node`s, which each have a representative ID. The nodesystem will allow you to add nodes to the internal node tree by way of functions. 

Currently this crate/module is in a very fledgling state, however the end goal is to implement:
- 2D graphics;
- A full ECS;
- Asset loading capabilities;
- Audio capabilities;
- Texture loading;

But I am alas but one person with varying degrees of motivation throughout the day.

Here is an example of the current syntax:

```python
from ferrokrait import *

def hello_world(app):
  print("Hello, World!")
  
def print_delta(app):
  print(app.get("delta"))

NodeSystem() \
  .add_node() \
  .with_field("data of any kind", "this data's name") \
  .add_startup_system(hello_world) \
  .add_system(print_delta) \
  .run() \

```
The above code is currently broken due to some borrow checking errors in my code. 

There is a little to unpack here so lets cover it together.
- `NodeSystem()`: The base App class, holding all ECS data;
- `new_node()`: Creates a new `Node`, which is an object that points to some data;
- `with_field(...)`: Adds any kind of data to the node childen;
- `add_system(...)`: Adds a system (function) to the app;
- `add_startup_system(...)`: Adds a system (function) to the app which executes on init;
- `run()`: Executes the app;
- `get(...)`: Gets data from the app;

How to support:
  - Test trial the module
  - Complain to me about bugs
  - Make suggestions
  - Contribute and write code
