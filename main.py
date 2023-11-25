import markovify

# Get raw text as string.
with open("filtered_noumena.txt") as f:
    text = f.read()

# Build the model.
text_model = markovify.Text(text, state_size=3, well_formed=False)

# Print five randomly-generated sentences
for i in range(25):
    print(text_model.make_short_sentence(300, tries=50, max_overlap_ratio=0.5, max_overlap_total=10))

# Print three randomly-generated sentences of no more than 280 characters
# for i in range(3):
#     print(text_model.make_short_sentence(280))