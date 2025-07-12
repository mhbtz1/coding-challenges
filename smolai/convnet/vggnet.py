from tqdm import tqdm
import torch
import torch.nn as nn
import datasets
import numpy as np
from torchvision.datasets import MNIST
from torchvision.transforms import transforms
import matplotlib.pyplot as plt
from dataclasses import dataclass

@dataclass
class TrainConfiguration:
    H = 28
    W = 28

class VGGNet(nn.Module):
    def __init__(self):
        super().__init__()
        self.feature_sampler = torch.nn.Sequential(
            nn.Conv2d(in_channels=1, out_channels=3, kernel_size=2),
            nn.Conv2d(in_channels=3, out_channels=6, kernel_size=4),
            nn.MaxPool2d(kernel_size=2),
            nn.ReLU(),
            nn.Conv2d(in_channels=6, out_channels=12, kernel_size=4),
            nn.Conv2d(in_channels=12, out_channels=48, kernel_size=2),
            nn.MaxPool2d(kernel_size=2),
            nn.ReLU(),
            nn.Flatten()
        )


        '''
        self.flatten = nn.Flatten()
        inp = torch.zeros(1, 1, TrainConfiguration.H, TrainConfiguration.W)
        out = self.flatten(self.feature_sampler(inp))
        shp = out.size(1)
        '''

        self.classifier = torch.nn.Sequential(
            nn.Linear(in_features=768, out_features=224),
            nn.ReLU(),
            nn.Linear(in_features=224, out_features=100),
            nn.Linear(in_features=100, out_features=20),
            nn.ReLU(),
            nn.Linear(in_features=20, out_features=10)

        )

    def forward(self, x):
        rpr = self.feature_sampler(x)
        logits = self.classifier(rpr)
        logits = torch.nn.functional.softmax(logits, dim=1)
        return logits
    




def train(model: VGGNet, train_dataloader: torch.utils.data.Dataset, optimizer: torch.optim.Adam, loss: torch.nn, epochs: int = 5):
    for epoch in tqdm(range(epochs)):
        avg_loss = 0.0
        for i, (input, label) in enumerate(train_dataloader):
            optimizer.zero_grad()
            output = model(input)

            #print(f"label: {label}")
            label_tensor = torch.nn.functional.one_hot(label, num_classes=10)
            #print(f"label_tensor: {label_tensor}")

            fn_loss = loss(output.float(), label_tensor.float())
            avg_loss += fn_loss.item()
            if i > 0 and i % 100 == 0:
                print("Avg. loss on epoch {} on step {}: {}".format(epoch, i, round(avg_loss / float(i), 3)))
            fn_loss.backward()
            optimizer.step()


def test(model: VGGNet, test_dataloader: torch.utils.data.DataLoader):
    with torch.inference_mode(mode=True):
        acc = 0.0
        for i, (output, label) in test_dataloader:
            gen_output = model(output)
            gen_label = torch.argmax(gen_output)
            if gen_label == label:
                acc += 1
        
        return acc / float(len(test_dataloader))


if __name__ == "__main__":
    loss = nn.CrossEntropyLoss()
    model = VGGNet()
    optimizer = torch.optim.Adam(model.parameters(), lr=0.01)

    train_dataset = MNIST(root="./data", train=True, transform=transforms.ToTensor(), download=True)
    test_dataset = MNIST(root="./data", train=False, transform=transforms.ToTensor(), download=True)

    train_dataloader = torch.utils.data.DataLoader(train_dataset, batch_size=32, shuffle=True)
    test_dataloader = torch.utils.data.DataLoader(test_dataset, batch_size=32, shuffle=True)

    input, label = train_dataset[0]
    input1, label1 = train_dataset[1]

    inp_tensor = torch.cat([torch.unsqueeze(input, dim=0), torch.unsqueeze(input1, dim=0)], dim=0)

    print(inp_tensor.size())
    x = model(inp_tensor)
    print(x)

    labels = torch.Tensor([label, label1]).long()
    print(labels)
    print(torch.nn.functional.one_hot(labels, num_classes=10))

    fn_loss = loss(x, labels)
    print(fn_loss)

    #train(model=model, loss=loss, optimizer=optimizer, train_dataloader=train_dataloader)
    #test(model=model, test_dataloader=test_dataloader)

