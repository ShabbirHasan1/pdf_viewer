# Prime directives
- Explain thing first before doing them
- Provide an assestment
- Give me 3 different options and let me choose

# Base tools to use in the project
- Rust
- egui

# Problem Statement
I am a mathematician and would like to explore how probability density functions behave when they are multiplied together. I would like a graphical tool that will allow me to explore how the resultant probability density function changes as the input probability density functions change. I need to be able to visualize 1 and 2 dimensional PDFs. I am concerned with gaussian distributions at this time, however in the future I may want to explore other distributions as well and I would like to easily change the distributions. I want a tool that is very fast and can update the plots in real time with no noticeble lag.

# Functional Requirements
The pdf_viewer must:
- allow the user to specify the input distribution parameters
- allow the user to add new distributions to the tool
- allow the user to select two distributions to multiply
- allow the user to multiply the product of two distributions with another distribution
- keep track of the dependency of a given pdf on all parental pdfs
- allow the user to delete one or more seleted pdfs
- notify the user and ask for confirmation when deleting pdfs
- keep track of the parental distribution parameters so that if a parent is deleted, the values used to generate the distribution are still available
- allow the user to dynamically adjust parental distribution parameters using a uicontrol such as a slider or knob
- allow the user to dynamically adjust parental distribution parameters by entering a number directly
- keep uicontrol and numerical boxes in sync with each other
- allow the user to save a session for later
- allow the user to load a saved session
- allow the user to change the zoom of the display axes
- allow the user to pan the display axes
- allow the user to reset the view to show all pdfs
- provide a keyboard centric approach to interacting with all controls
- allow the user to also interact with a mouse / touchpad
- provide shading under the curves with variable user set opacity
- allow the user to mark important regions like 1,2 and 3 standard deviations
- allow the user to toggle shading and markings
- allow the user to primarily view the tool in an egui window
- provide secondary support for a locally served static website (e.g. zora)

* Other directives
- Keep a log of changes to the program as we iterate in PROJECT.md