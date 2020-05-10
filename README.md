# Astroflux Rust Architecture

## Summary of Mathematics

1. Let $\textbf S \in \mathbb R^{N^2 \times 2}$ be the matrix of samples from the antenna beam in the $(l,m)$ plane corresponding to a square image of $N \times N$ pixels, i.e., the stacked row vectors of $(l,m)$ coordinates.
2. Let $\textbf Z_{ant} \in \mathbb R^{J \times 2}$ be the matrix of $(u,v)$ coordinates for $J$ antennas
3. The complex phase $\boldsymbol{\Phi} \in \mathbb C^{J \times N^2}$ of each antenna's input pixels is then

$$
\boldsymbol{\Phi} = e^{-2 \pi j \textbf Z_{ant} \textbf S^\top}
$$

4. Let $\textbf p \in \mathbb R^{N^2}$ be the vector of input pixel values
5. The received antenna signals vector $\textbf x \in \mathbb C^{J}$ is

$$
\textbf x = \boldsymbol{\Phi}\textbf p
$$

6. The cross correlation $\textbf R \in \mathbb R^{J \times J}$ is then

$$
\textbf R = \textbf x \textbf x^H
$$

7. We reshape this into a vector $\textbf r \in \mathbb R^{J^2}$
8. Let $\textbf Z_{bl} \in \mathbb R^{J^2 \times 2}$ be the matrix of $(u,v)$ baselines among the antennas 

$$
\begin{align}
\textbf Z_{ant,u} &= 
\begin{bmatrix}
\text{col}_1 (\textbf Z_{ant})
&
\cdots
\end{bmatrix}_{J\times J}
\\
\textbf Z_{ant,v} &= 
\begin{bmatrix}
\text{col}_2 (\textbf Z_{ant})
&
\cdots
\end{bmatrix}_{J\times J}
\\
\textbf Z_{bl,u} &= \textbf Z_{ant,u} - \textbf Z_{ant,u}^\top
\\
\textbf Z_{bl,v} &= \textbf Z_{ant,v} - \textbf Z_{ant,v}^\top
\\
\textbf Z_{bl} &= \begin{bmatrix}
\text{flatten}(\textbf Z_{bl,u}) &
\text{flatten}(\textbf Z_{bl,v})
\end{bmatrix}
\end{align}
$$

9. The complex phase adjustment $\boldsymbol \Theta \in \mathbb C^{J^2 \times N^2}$ for each baseline is

$$
\boldsymbol{\Theta} = e^{2 \pi j \textbf Z_{bl} \textbf S^\top}
$$

10. The complex dirty image pixels vector is $\textbf p'_D \in \C^{N^2}$

$$
\textbf p'_D = \textbf r \boldsymbol{\Theta}
$$

11. This is more useful as the intensity values $\textbf p_D \in R^{N^2}$

$$
\textbf p_D = |\textbf p'_D|^2
$$

12. To get the final dirty image, reshape into an $N \times N$ vector

## Data Entities

- $\textbf S$, the matrix of $(l,m)$ samples for the antenna beam
- $\textbf Z_{ant}$, the matrix of antenna $(u,v)$ coordinates
- $\boldsymbol {\Phi}(\textbf Z, \textbf S)$, beam sample phases for either the baselines or antenna positions
- $\textbf r$, the cross correlation output

```rust

```



```rust
// Assume we have Image, AntennaArray
let N = 64;
let beam = Beam::from_beamwidth(beamwidth, N);
let antenna_uv = array.as_uv(wavelength);
let antenna_beam_phases = beam.phases(&antenna_uv);
let image_pixels = image.to_vector();
let rx: AntennaSignals;
rx.signals = antenna_beam_phases.dot(&image_pixels.insert_axis(Axis(1)));
let xcorr = rx.xcorr();
let baselines = array.baselines(wavelength);
let baseline_phases = beam.phases(&baslines);
let dirty_image_pixels = flatten(xcorr.dot(&baseline_phases));
let dirty_image = Image::from_vector(dirty_image_pixels);
```

