from ferrokrait import *

# A simple node that derives the Node class.
class TestNode(Node):
    def _ready(self):  
        print("Hello,")
        
# Subclasses are usable too!
class TestNodeSubclass(TestNode):
    def _ready(self):
        print("World!")

# NodeTree ( new ), add node ( TestNodeSubclass ), run ( at a 60 fps cap )
NodeTree().add_node(TestNodeSubclass).run(60)
