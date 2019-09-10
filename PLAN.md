
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



