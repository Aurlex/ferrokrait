from ferrokrait import *

class InputTest(Node):
    def _on_key_input(self):
        print("guau", end="")
    def _process(self, delta):
        print("")

get_tree().add_node(InputTest).run(60)