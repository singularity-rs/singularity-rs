# Overall Description

The Initial basic idea is to start out with something similar to Achikaps first.

## Achikaps
You start out with some Platforms. Platforms are connected with Roads, forming
a Graph. On this Graph, Units can travel. Units can do several Tasks, including
'Produce', 'Carry', 'Buid' and 'Defend'. Some Platforms are 'Producing'
platforms, meaning, if is a Unit on this platform 'producing' resources (and
the required conditions are fulfilled), in specific interavals as many
resources appear as there are Units 'producing'. 'Carry' describes the carrying
of Resources from one Platform to another, typically for refining (transforming
a combination of resources in another), or moving them to the storage. In
Achikaps, this also includes 'refilling' of Batteries for Laser-defending
towers or power stations (which require new batteries after some shots or time).
Building is basically the same as carrying, however Resources are only carried
for the building of new Platforms or other structures (roads, ...). Building
platforms happens by creating a blueprint through the build-menu, after which 
several Tasks for building units are created to transport the required resources.
New Units can be created by building 'Houses' (also Platforms) and providing it
with the required resources (usually five not that complex ones). These are then
carried by 'Carrying' Units, after they received their tasks.

Aforementioned Power Stations provide 'Energy' for all platforms in their
vicinity. Platforms will not create tasks if they require power but don't have
any (such as producing facilities, Houses, ...). Power stations have tasks for
Carrying units to get them batteries, but this only works as long as there are
batteries available. Batteries are produced in a windmill.

Some of the Production processes require the transportation of resources across
multiple 'Factory'-platforms to transform them. Initial Resources are 'Pearls'
(from a 'Factory'), 'Metal' (from a 'Mine', but only if there is a deposit with
metal next to it). 'Meat' can also be produced with further restraints, by
'Farms'. The 'Meat Grinder' requires two Meat and one Metal to produce one
'Food', Houses require four 'Pearls' and one 'Food' for each new Unit produced.
One House can only produce three new Units. However, if the produced units die,
new ones can be produced again. A Laboratory requires one Pearl, one Food and
one Meatto produce one chewing gum.

There is also Defensive Platforms, some of which need to be operated. They
provide 'Defend'-type tasks for units, and only defend when there are Units
operating them. Their firepower also depends on the units present. Other
platforms provide firepower independent of Units, as long as Batteries are
delivered ('Laser' towers). In some levels, you need to fend of several waves,
with 20 not being uncommon. Defeated Enemies sometimes leave some Metal on the
next platform (if it is not full already). In some levels, these resources
are needed since metal is rare sometimes (as it needs a deposit for a mine).

Recently some more features got added to Achikaps, which I may or may not
explain in detail, but are not necessary for the core understanding. One added
part is a slightly more complex military-industrial part and some increased
efficiency for transporting resources, Boosts, and more.

## Own Ideas

First, I'd like to include just about everything described so far, though maybe
renaming and/or recoloring the resources. I absolutely like the Graph-based
idea, also the 'easiness' for producing some, but not all resources. I also
like how work is distributed and such. I had some hassles with the timing of
defending, but I think about keeping that as well. Something annoying
(especially when building larger structures) was that you need to build each
platform seperately. There is a 'faster' building mode, but I would like to
create bigger (staged) blueprints as well (like one platform surrounded only by
houses, with one connection to another platform). When placing Platforms, one
is free to choose where to set it. It might be worth experimenting with fixed
degrees instead (8 Platforms -> 45°, 12 -> 30°, 6 -> 60°). I think eight is a
good number.





# Detailed Required Parts

<br/><br/>
## Template
<br/><br/>

- [ ] Entities
    - [ ] Component
<br/><br/>
    - [ ] Optional Component
<br/><br/>
    - [ ] Component (not MVP)

<br/><br/>
- [ ] System
    - Required System
    - Associated Component
    - Needed Entity




<br/><br/>
## General, Core Game functionality
<br/><br/>

- [ ] Unit
    - [ ] Moving
        - [ ] Position
        - [ ] Velocity
        - [ ] Acceleration
    - [ ] UnitType (Produce, Carry, Build, Defend)
    - [ ] Graph-Based Pathfinding module
    - [ ] Communication: Getting Tasks (finishing, failing, impossible to do, ...)
    - [ ] Drawable
<br/><br/>
    - [ ] ResourceCarrying
    - [ ] ActiveTask

<br/><br/>

- [ ] Tasks
    - [ ] JobType
    - [ ] BeginPlatform
    - [ ] EndPlatform
    - [ ] DoWorkForPlatformAction
    - [ ] ResourceType


<br/><br/>

- [ ] FreeMovingUnit (not MVP)
    - [ ] Moving
        - [ ] Position
        - [ ] Velocity
        - [ ] Acceleration
    - [ ] A\*-based pathfinding
    - [ ] Flocking / FlockingGroups
        - [ ] FlockingGroupNumber
        - [ ] GroupMiddle
        - [ ] GroupAverage
    - [ ] TeamNumber (own or enemy/other faction)
    - [ ] ReportState
    - [ ] Selectable
    - [ ] Drawable
    - [ ] LoseHealth
<br/><br/>
    - [ ] Shooting
    - [ ] TargetingAt
    - [ ] UserControl
    - [ ] AIControl
<br/><br/>
    - [ ] Settler (not MVP)
    - [ ] Transporter (not MVP)
    - [ ] Communicator (not MVP)


<br/><br/>

- [ ] StoryManager
    - [ ] Level
        - [ ] Goals
        - [ ] Enemies
    - [ ] Map
        - [ ] Fog Of War
        - [ ] CollisionMap
            - [ ] Collidable objects on Map
        - [ ] ResourceMap
        - [ ] StructureMap
        - [ ] UnitMap
        - [ ] Camera (? -> put that somewhere else !!)


<br/><br/>

- [ ] Graph
    - [ ] Nodes
    - [ ] Edges


<br/><br/>

- [ ] Platform
    - [ ] PlatformType
    - [ ] Position
    - [ ] ResourcesOnPlatform
    - [ ] IsBluePrint
    - [ ] Selectable
    - [ ] ReportState
    - [ ] PauseActions
    - [ ] LoseHealth
    - [ ] Drawable
    - [ ] ConnectedPlatforms
    - [ ] RoutingTable
<br/><br/>
    - [ ] ProduceResource<Which>
    - [ ] RequestResources
    - [ ] RemoveResource
    - [ ] CreateResource
    - [ ] DefendWithBullets
    - [ ] DefendWithLaser
    - [ ] RequestUnits
<br/><br/>
    - [ ] DefendWithRockets (not MVP)
    - [ ] Communicator (not MVP)
    - [ ] CreateEnergy (not MVP)
    - [ ] StoreResource (not MVP) (-> Logistics Hub)
    - [ ] CreateUnit (not MVP)
    - [ ] CreateFreeMovingUnit (not MVP)


<br/><br/>

## Graphics and User Interface
<br/><br/>


- [ ] Camera
    - [ ] Position
    - [ ] Size (Viewport)
    - [ ] Zoomlevel
    - [ ] Received Input



<br/><br/>


- [ ] GraphUnitStats
    - [ ] Position
    - [ ] Size
    - [ ] Label(s)
    - [ ] GraphSelector
    - [ ] UnitTypeField
        - [ ] Label
        - [ ] HorizontalSlider
        - [ ] Number


<br/><br/>

- [ ] ResourceStats
    - [ ] Position
    - [ ] Size
    - [ ] ResourceDataCounter
    - [ ] ResourcePresenter
        - [ ] ResourceGraphic
        - [ ] NameLabel
        - [ ] NumberLabel
        - [ ] DeltaLabel
    - [ ] VerticalSlider


<br/><br/>

- [ ] SelectionStats
    - [ ] Position
    - [ ] Size
    - [ ] Label
    - [ ] ResourcePresenter
        - [ ] ResourceGraphic
        - [ ] NameLabel
        - [ ] NumberLabel
    - [ ] ActionPresenter
        - [ ] ActionGraphic
        - [ ] ActionLabel
        - [ ] ResourceReqGraphics
    - [ ] UnitPresenter
        - [ ] UnitTaskGraphic
        - [ ] ...
    - [ ] FreeMovingUnitPresenter
        - [ ] UnitGraphic
        - [ ] ...
    - ReportState(s)


<br/><br/>

- [ ] EventLog
    - [ ] Position
    - [ ] Size
    - [ ] ...


<br/><br/>

- [ ] NextGoals
    - [ ] Position
    - [ ] Size
    - [ ] ...


<br/><br/>

- [ ] BuildMenu
    - [ ] Position
    - [ ] Size
    - [ ] ...


<br/><br/>

- [ ] MiniMap (not MVP)
    - [ ] Position
    - [ ] Size
    - [ ] ...



<br/><br/>
## Screens
<br/><br/>

- [ ] MainMenuScreen
    - UI elements
        - Labels
        - Buttons
- [ ] GameScreen
    - [ ] Camera
    - [ ] Map(s)
    - UI elements
        - GraphUnitStats
        - ResourceStats
        - SelectionStats
        - EventLog
        - NextGoals
        - BuildMenu
        - MiniMap (not MVP)
- [ ] PauseScreen
    - UI elements
        - Labels
        - Buttons

<br/><br/>
- [ ] LoadGameScreen (not MVP)
    - UI elements
        - Labels
        - Buttons
- [ ] Options Screen (not MVP)
    - UI elements
        - Labels
        - Buttons
        - HorizontalSlider
        - VerticalSlider
        - RadioBoxes

- [ ] TutorialScreen (not MVP)
    - UI elements
        - Labels
        - Buttons
    






<br/><br/>
<br/><br/>


## Systems
<br/><br/>

- [ ] Setting Blueprints
    - Platforms
    - Road (?)
    - Graph
    - DistributionManager -> notification: ready to build
    - Map
    - BuildMenu
- [ ] Building Blueprints
    - Setting Blueprints
    - Units
    - Tasks
    - Resources
    - Paths / Roads
    - DistributionManager -> creating tasks for units to put resources there
- [ ] Selecting Platforms
    - Platforms
    - Map/StructureMap
    - Camera
    - SelectionStats
    - Catching Input for specific Platform correctly
- [ ] Update resource numbers on Platforms
    - Part 1:
        - Resources
        - Time
        - CreateResource
    - Part 2:
        - Resource
        - Units
        - Tasks
    - Actually necessary?
- [ ] Making Sliders work for how many units should do what on which Graph
    - GraphUnitStats
    - DistributionManager
    - Probably only a channel, not a System
- [ ] Updating all the UI elements
    - All the UI elements
    - ...
    - some other way in amethyst?
- [ ] Moving & adjusting Camera
    - Input-catching stuff
    - GameScreen
    - Camera
- [ ] Pause / Unpause Game
    - Input-catching stuff
    - PauseScreen
    - Buttons
- [ ] Update on achieved goals/trigger next goals/trigger Enemies coming/ ....
    - Extensive logging all over the place
    - guess this needs to be split up more ...
<br/><br/>
- [ ] Moving (Free-) Units and Resources
    - Map(s)
    - Units
    - FreeMovingUnits
    - Resources
    - Time-based Update-cycles
- [ ] Do Path-finding for these needed
    - Map(s)
    - PathFinding Algorithms implemented
    - Units
    - FreeMovingUnits
- [ ] Flocking (not MVP, because FMU not MVP)
    - [ ] Compute direction averages
    - [ ] Compute middle points
    - [ ] Compute distance between units
    - FreeMovingUnits
    - Map(s)
<br/><br/>
- [ ] Produce & Consume Resources
    - Resources
    - (producing) Platforms
    - ResourceMap
- [ ] Transport Resources
    - Units
    - DistributionManager
    - Graph
    - GraphPathPlanning
    - Resources
    - Platforms
    - Roads
    - Tasks
    - System: Movement
- [ ] Provide new Tasks for Units asking for them -> DistributionManager
    - [ ] JoinGraphs
    - [ ] SplitGraphs
    - [ ] AddToGraph
    - [ ] Calculate flowgraphs of resources (async? MVP?)
    - Units
    - Tasks
    - Resources
    - Platforms
    - Roads
    - Graph
    - GraphPathPlanning
    - Channels for communication
<br/><br/>
- [ ] 'StoryManager' for advancing the story / levels upon reaching goals
    - Map(s)
    - Level(s)
    - Goals
    - Loading level descriptions
    - ReportState(s)
- [ ] Planning Military maneuvers from AI (-> MilitaryManager; not MVP)
    - FreeMovingUnit
    - TeamNumber
    - Map(s)
- [ ] Planning Building and Resource aquiring from AI
    - DistributionManager
    - MilitaryManager


<br/><br/>
<br/><br/>

## Other stuff
<br/><br/>

- [ ] Flocking (https://en.wikipedia.org/wiki/Boids, https://github.com/henninglive/boids-rs/blob/master/src/main.rs )
- [ ] Agent-like behaviour
- [ ] Behaviour-trees
- [ ] Model-View-Controller (splitting)
- [ ] Procedural Map generation: http://pcg.wikidot.com/category-pcg-algorithms
- [ ] Blueprint Editor
- [ ] Everything not in MVP

<br/><br/>
- [ ] Plan it all out before writing code (in progress). Update as needed.
- [ ] decouple graphics itself quite intensely from everything else
- [ ] Actually, adhere to 'Model-View-Controller' schema
- [ ] make it simple to make a 3D-version as well (plan that one)
- [ ] Make fixed steps of 45° and lengths of [1, 2, 3] * fixed_length
- [ ] Do the Amethyst tutorial.



