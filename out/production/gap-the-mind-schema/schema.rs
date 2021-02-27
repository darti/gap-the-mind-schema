/// A comment on an item - for example, a comment on a blog post. The comment's content is expressed via the [[text]] property, and its topic via [[about]], properties shared with all CreativeWorks.
struct Comment;

/// The service provided by this channel.
struct providesService;

/// An item is an object within the game world that can be collected by a player or, occasionally, a non-player character.
struct gameItem;

/// BasicIncome: this is a benefit for basic income.
struct BasicIncome;

/// A Hindu temple.
struct HinduTemple;

/// A sub property of object. The candidate subject of this action.
struct candidate;

/// Represents EU Energy Efficiency Class F as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryF;

/// A public toilet is a room or small building containing one or more toilets (and possibly also urinals) which is available for use by the general public, or by customers or employees of certain businesses.
struct PublicToilet;

/// The direct performer or driver of the action (animate or inanimate). e.g. *John* wrote a book.
struct agent;

/// A maintainer of a [[Dataset]], software package ([[SoftwareApplication]]), or other [[Project]]. A maintainer is a [[Person]] or [[Organization]] that manages contributions to, and/or publication of, some (typically complex) artifact. It is common for distributions of software and data to be based on "upstream" sources. When [[maintainer]] is applied to a specific version of something e.g. a particular version or packaging of a [[Dataset]], it is always  possible that the upstream source has a different maintainer. The [[isBasedOn]] property can be used to indicate such relationships between datasets to make the different maintenance roles clear. Similarly in the case of software, a package may have dedicated maintainers working on integration into software distributions such as Ubuntu, as well as upstream maintainers of the underlying work.
///       
struct maintainer;

/// A painting.
struct Painting;

/// Natural languages such as Spanish, Tamil, Hindi, English, etc. Formal language code tags expressed in [BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) can be used via the [[alternateName]] property. The Language type previously also covered programming languages such as Scheme and Lisp, which are now best represented using [[ComputerLanguage]].
struct Language;

/// A publication containing information about varied topics that are pertinent to general information, a geographic area, or a specific subject matter (i.e. business, culture, education). Often published daily.
struct Newspaper;

/// The URL that goes directly to the plan brochure for the specific standard plan or plan variation.
struct healthPlanMarketingUrl;

/// Real or fictional location of the game (or part of game).
struct gameLocation;

/// A technical article - Example: How-to (task) topics, step-by-step, procedural troubleshooting, specifications, etc.
struct TechArticle;

/// Symptoms or related symptoms of a Topic.
struct SymptomsHealthAspect;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserBlocks;

/// A florist.
struct Florist;

/// A delivery service through which content is provided via broadcast over the air or online.
struct BroadcastService;

/// Tracking url for the parcel delivery.
struct trackingUrl;

/// The act of un-registering from a service.\n\nRelated actions:\n\n* [[RegisterAction]]: antonym of UnRegisterAction.\n* [[LeaveAction]]: Unlike LeaveAction, UnRegisterAction implies that you are unregistering from a service you werer previously registered, rather than leaving a team/group of people.
struct UnRegisterAction;

/// The 25th percentile value.
struct percentile25;

/// A single message from a sender to one or more organizations or people.
struct Message;

/// AlbumRelease.
struct AlbumRelease;

/// MerchantReturnNotPermitted: product returns are not permitted.
struct MerchantReturnNotPermitted;

/// The date of production of the item, e.g. vehicle.
struct productionDate;

/// A map.
struct Map;

/// PaidLeave: this is a benefit for paid leave.
struct PaidLeave;

/// A program with both an educational and employment component. Typically based at a workplace and structured around work-based learning, with the aim of instilling competencies related to an occupation. WorkBasedProgram is used to distinguish programs such as apprenticeships from school, college or other classroom based educational programs.
struct WorkBasedProgram;

/// A possible treatment to address this condition, sign or symptom.
struct possibleTreatment;

/// The human sensory perceptual system or cognitive faculty through which a person may process or perceive information. Expected values include: auditory, tactile, textual, visual, colorDependent, chartOnVisual, chemOnVisual, diagramOnVisual, mathOnVisual, musicOnVisual, textOnVisual.
///       
struct accessMode;

/// The name of the application suite to which the application belongs (e.g. Excel belongs to Office).
struct applicationSuite;

/// Results are not available.
struct ResultsNotAvailable;

/// A specific branch of medical science that is concerned with poisons, their nature, effects and detection and involved in the treatment of poisoning.
struct Toxicologic;

/// A library.
struct Library;

/// AuthenticMediaObject: An unaltered image that is presented in an accurate way.
struct AuthenticContent;

/// Any description of pages that is not separated into pageStart and pageEnd; for example, "1-6, 9, 55" or "10-12, 46-49".
struct pagination;

/// A sporting goods store.
struct SportingGoodsStore;

/// A broadcast service to which the broadcast service may belong to such as regional variations of a national channel.
struct parentService;

/// DigitalFormat.
struct DigitalFormat;

/// The creator/author of this CreativeWork. This is the same as the Author property for CreativeWork.
struct creator;

/// The range of temporal applicability of a dataset, e.g. for a 2011 census dataset, the year 2011 (in ISO 8601 time interval format).
struct datasetTimeInterval;

/// A set of links that can help a user understand and navigate a website hierarchy.
struct breadcrumb;

/// Indicates a specific [[CivicStructure]] or [[LocalBusiness]] associated with the SpecialAnnouncement. For example, a specific testing facility or business with special opening hours. For a larger geographic region like a quarantine of an entire region, use [[spatialCoverage]].
struct announcementLocation;

/// The location (e.g. civic structure, local business, etc.) where a person can go to access the service.
struct serviceLocation;

/// An organization that acknowledges the validity, value or utility of a credential. Note: recognition may include a process of quality assurance or accreditation.
struct recognizedBy;

/// Proprietary name given to the diet plan, typically by its originator or creator.
struct proprietaryName;

/// Health and beauty.
struct HealthAndBeautyBusiness;

/// Indicates whether this game is multi-player, co-op or single-player.  The game can be marked as multi-player, co-op and single-player at the same time.
struct playMode;

/// Defines the energy efficiency Category (also known as "class" or "rating") for a product according to an international energy efficiency standard
struct hasEnergyConsumptionDetails;

/// An application that can complete the request.
struct application;

/// An EventAttendanceModeEnumeration value is one of potentially several modes of organising an event, relating to whether it is online or offline.
struct EventAttendanceModeEnumeration;

/// The opening hours of a certain place.
struct openingHoursSpecification;

/// Nonprofit501c18: Non-profit type referring to Employee Funded Pension Trust (created before 25 June 1959).
struct Nonprofit501c18;

/// The time interval used to compute the invoice.
struct billingPeriod;

/// A pointer to a secondary value that provides additional information on the original value, e.g. a reference temperature.
struct valueReference;

/// The amount of time that is required between accepting the offer and the actual usage of the resource or service.
struct advanceBookingRequirement;

/// An enumeration that describes different types of medical procedures.
struct MedicalProcedureType;

/// The material used. (e.g. Oil, Watercolour, Acrylic, Linoprint, Marble, Cyanotype, Digital, Lithograph, DryPoint, Intaglio, Pastel, Woodcut, Pencil, Mixed Media, etc.)
struct artMedium;

/// The category of the recipe—for example, appetizer, entree, etc.
struct recipeCategory;

/// Suspended.
struct Suspended;

/// The units of an active ingredient's strength, e.g. mg.
struct strengthUnit;

/// A media object representing the circumstances before performing this direction.
struct beforeMedia;

/// Audiences defined by a person's gender.
struct requiredGender;

/// Prerequisites needed to fulfill steps in article.
struct dependencies;

/// A specific branch of medical science that pertains to diagnosis and treatment of disorders of muscles, ligaments and skeletal system.
struct Musculoskeletal;

/// Indicates an item funded or sponsored through a [[Grant]].
struct fundedItem;

/// The number of grams of saturated fat.
struct saturatedFatContent;

/// The frequency in MHz and the modulation used for a particular BroadcastService.
struct BroadcastFrequencySpecification;

/// An item used as either a tool or supply when performing the instructions for how to to achieve a result.
struct HowToItem;

/// Nonprofit501c5: Non-profit type referring to Labor, Agricultural and Horticultural Organizations.
struct Nonprofit501c5;

/// Identifier of the flight's departure gate.
struct departureGate;

/// A HyperToc represents a hypertext table of contents for complex media objects, such as [[VideoObject]], [[AudioObject]]. Items in the table of contents are indicated using the [[tocEntry]] property, and typed [[HyperTocEntry]]. For cases where the same larger work is split into multiple files, [[associatedMedia]] can be used on individual [[HyperTocEntry]] items.
struct HyperToc;

/// The airline-specific indicator of boarding order / preference.
struct boardingGroup;

/// Uses devices to support users with hearing impairments.
struct HearingImpairedSupported;

/// The quality of the video.
struct videoQuality;

/// The drug or supplement's legal status, including any controlled substance schedules that apply.
struct legalStatus;

/// The Person's occupation. For past professions, use Role for expressing dates.
struct hasOccupation;

/// A service provided by a government organization, e.g. food stamps, veterans benefits, etc.
struct GovernmentService;

/// For a [[NewsMediaOrganization]], a link to the masthead page or a page listing top editorial management.
struct masthead;

/// A link to the ListItem that preceeds the current one.
struct previousItem;

/// The act of producing a painting, typically with paint and canvas as instruments.
struct PaintAction;

/// A Consortium is a membership [[Organization]] whose members are typically Organizations.
struct Consortium;

/// Similar to courseMode, The medium or means of delivery of the program as a whole. The value may either be a text label (e.g. "online", "onsite" or "blended"; "synchronous" or "asynchronous"; "full-time" or "part-time") or a URL reference to a term from a controlled vocabulary (e.g. https://ceds.ed.gov/element/001311#Asynchronous ).
struct educationalProgramMode;

/// Specifies the most energy efficient class on the regulated EU energy consumption scale for the product category a product belongs to. For example, energy consumption for televisions placed on the market after January 1, 2020 is scaled from D to A+++.
struct energyEfficiencyScaleMax;

/// The number of upvotes this question, answer or comment has received from the community.
struct upvoteCount;

/// Position of the clip within an ordered group of clips.
struct clipNumber;

/// The total price for the reservation or ticket, including applicable taxes, shipping, etc.\n\nUsage guidelines:\n\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
struct totalPrice;

/// The day of the week for which these opening hours are valid.
struct dayOfWeek;

/// The quantity of the given bed type available in the HotelRoom, Suite, House, or Apartment.
struct numberOfBeds;

/// This ordering relation for qualitative values indicates that the subject is lesser than or equal to the object.
struct lesserOrEqual;

/// Either the actual menu as a structured representation, as text, or a URL of the menu.
struct menu;

/// Event type: Business event.
struct BusinessEvent;

/// Indicates that the item is used.
struct UsedCondition;

/// The amount of money.
struct amount;

/// Represents a relationship between two geometries (or the places they represent), relating a covering geometry to a covered geometry. "Every point of b is a point of (the interior or boundary of) a". As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoCovers;

/// A set of characteristics belonging to businesses, e.g. who compose an item's target audience.
struct BusinessAudience;

/// A single ingredient used in the recipe, e.g. sugar, flour or garlic.
struct recipeIngredient;

/// MixtapeAlbum.
struct MixtapeAlbum;

/// The latest date the package may arrive.
struct expectedArrivalUntil;

/// The place(s) from which the offer can be obtained (e.g. store locations).
struct availableAtOrFrom;

/// The act of producing/preparing food.
struct CookAction;

/// A short textual code that uniquely identifies the value.
struct codeValue;

/// The item being described is intended to assess the competency or learning outcome defined by the referenced term.
struct assesses;

/// Any rule set or interactive tool for estimating the risk of developing a complication or condition.
struct MedicalRiskEstimator;

/// A reservation for boat travel.
/// 
/// Note: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
struct BoatReservation;

/// The total number of forward gears available for the transmission system of the vehicle.\n\nTypical unit code(s): C62
struct numberOfForwardGears;

/// The server on which  it is possible to play the game.
struct gameServer;

/// Target audiences for medical web pages.
struct MedicalAudience;

/// OfferShippingDetails represents information about shipping destinations.
/// 
/// Multiple of these entities can be used to represent different shipping rates for different destinations:
/// 
/// One entity for Alaska/Hawaii. A different one for continental US.A different one for all France.
/// 
/// Multiple of these entities can be used to represent different shipping costs and delivery times.
/// 
/// Two entities that are identical but differ in rate and time:
/// 
/// e.g. Cheaper and slower: $5 in 5-7days
/// or Fast and expensive: $15 in 1-2 days
struct OfferShippingDetails;

/// The warranty promise(s) included in the offer.
struct warranty;

/// Defines the number of times a recurring [[Event]] will take place
struct repeatCount;

/// The album this is a release of.
struct releaseOf;

/// IATA identifier for an airline or airport.
struct iataCode;

/// A schematic image showing the floorplan layout.
struct layoutImage;

/// The causative agent(s) that are responsible for the pathophysiologic process that eventually results in a medical condition, symptom or sign. In this schema, unless otherwise specified this is meant to be the proximate cause of the medical condition, symptom or sign. The proximate cause is defined as the causative agent that most directly results in the medical condition, symptom or sign. For example, the HIV virus could be considered a cause of AIDS. Or in a diagnostic context, if a patient fell and sustained a hip fracture and two days later sustained a pulmonary embolism which eventuated in a cardiac arrest, the cause of the cardiac arrest (the proximate cause) would be the pulmonary embolism and not the fall. Medical causes can include cardiovascular, chemical, dermatologic, endocrine, environmental, gastroenterologic, genetic, hematologic, gynecologic, iatrogenic, infectious, musculoskeletal, neurologic, nutritional, obstetric, oncologic, otolaryngologic, pharmacologic, psychiatric, pulmonary, renal, rheumatologic, toxic, traumatic, or urologic causes; medical conditions can be causes as well.
struct MedicalCause;

/// A specific branch of medical science that specializes in the care of women during the prenatal and postnatal care and with the delivery of the child.
struct Obstetric;

/// A hospital with which the physician or office is affiliated.
struct hospitalAffiliation;

/// A ProductGroup represents a group of [[Product]]s that vary only in certain well-described ways, such as by [[size]], [[color]], [[material]] etc.
/// 
/// While a ProductGroup itself is not directly offered for sale, the various varying products that it represents can be. The ProductGroup serves as a prototype or template, standing in for all of the products who have an [[isVariantOf]] relationship to it. As such, properties (including additional types) can be applied to the ProductGroup to represent characteristics shared by each of the (possibly very many) variants. Properties that reference a ProductGroup are not included in this mechanism; neither are the following specific properties [[variesBy]], [[hasVariant]], [[url]]. 
struct ProductGroup;

/// The act of interacting with another person or organization.
struct InteractAction;

/// An enumeration of several kinds of Map.
struct MapCategoryType;

/// The geographic area where a service or offered item is provided.
struct areaServed;

/// The 90th percentile value.
struct percentile90;

/// An alternative, closely-related condition typically considered later in the differential diagnosis process along with the signs that are used to distinguish it.
struct DDxElement;

/// A property, used to indicate attributes and relationships of some Thing; equivalent to rdf:Property.
struct Property;

/// A short segment/part of a movie.
struct MovieClip;

/// An enumeration of genders.
struct GenderType;

/// A movie theater.
struct MovieTheater;

/// Represents EU Energy Efficiency Class B as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryB;

/// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
struct additionalType;

/// Indicates a MerchantReturnPolicy that may be applicable.
struct hasMerchantReturnPolicy;

/// A media object that encodes this CreativeWork. This property is a synonym for associatedMedia.
struct encoding;

/// The payee received the payment, but it was declined for some reason.
struct PaymentDeclined;

/// Auto body shop.
struct AutoBodyShop;

/// A media object that encodes this CreativeWork.
struct encodings;

/// The coding system, e.g. 'ICD-10'.
struct codingSystem;

/// Enumerates energy efficiency levels (also known as "classes" or "ratings") and certifications that are part of several international energy efficiency standards.
struct EnergyEfficiencyEnumeration;

/// The job title of the person (for example, Financial Manager).
struct jobTitle;

/// An amusement park.
struct AmusementPark;

/// A short TV program or a segment/part of a TV program.
struct TVClip;

/// The person or organization who wrote a composition, or who is the composer of a work performed at some event.
struct composer;

/// An in-progress action (e.g, while watching the movie, or driving to a location).
struct ActiveActionStatus;

/// Last postal code in the range (included). Needs to be after [[postalCodeBegin]].
struct postalCodeEnd;

/// The date that payment is due.
struct paymentDueDate;

/// A SpecialAnnouncement combines a simple date-stamped textual information update
///       with contextualized Web links and other structured data.  It represents an information update made by a
///       locally-oriented organization, for example schools, pharmacies, healthcare providers,  community groups, police,
///       local government.
/// 
/// For work in progress guidelines on Coronavirus-related markup see [this doc](https://docs.google.com/document/d/14ikaGCKxo50rRM7nvKSlbUpjyIk2WMQd3IkB1lItlrM/edit#).
/// 
/// The motivating scenario for SpecialAnnouncement is the [Coronavirus pandemic](https://en.wikipedia.org/wiki/2019%E2%80%9320_coronavirus_pandemic), and the initial vocabulary is oriented to this urgent situation. Schema.org
/// expect to improve the markup iteratively as it is deployed and as feedback emerges from use. In addition to our
/// usual [Github entry](https://github.com/schemaorg/schemaorg/issues/2490), feedback comments can also be provided in [this document](https://docs.google.com/document/d/1fpdFFxk8s87CWwACs53SGkYv3aafSxz_DTtOQxMrBJQ/edit#).
/// 
/// 
/// While this schema is designed to communicate urgent crisis-related information, it is not the same as an emergency warning technology like [CAP](https://en.wikipedia.org/wiki/Common_Alerting_Protocol), although there may be overlaps. The intent is to cover
/// the kinds of everyday practical information being posted to existing websites during an emergency situation.
/// 
/// Several kinds of information can be provided:
/// 
/// We encourage the provision of "name", "text", "datePosted", "expires" (if appropriate), "category" and
/// "url" as a simple baseline. It is important to provide a value for "category" where possible, most ideally as a well known
/// URL from Wikipedia or Wikidata. In the case of the 2019-2020 Coronavirus pandemic, this should be "https://en.wikipedia.org/w/index.php?title=2019-20\_coronavirus\_pandemic" or "https://www.wikidata.org/wiki/Q81068910".
/// 
/// For many of the possible properties, values can either be simple links or an inline description, depending on whether a summary is available. For a link, provide just the URL of the appropriate page as the property's value. For an inline description, use a [[WebContent]] type, and provide the url as a property of that, alongside at least a simple "[[text]]" summary of the page. It is
/// unlikely that a single SpecialAnnouncement will need all of the possible properties simultaneously.
/// 
/// We expect that in many cases the page referenced might contain more specialized structured data, e.g. contact info, [[openingHours]], [[Event]], [[FAQPage]] etc. By linking to those pages from a [[SpecialAnnouncement]] you can help make it clearer that the events are related to the situation (e.g. Coronavirus) indicated by the [[category]] property of the [[SpecialAnnouncement]].
/// 
/// Many [[SpecialAnnouncement]]s will relate to particular regions and to identifiable local organizations. Use [[spatialCoverage]] for the region, and [[announcementLocation]] to indicate specific [[LocalBusiness]]es and [[CivicStructures]]. If the announcement affects both a particular region and a specific location (for example, a library closure that serves an entire region), use both [[spatialCoverage]] and [[announcementLocation]].
/// 
/// The [[about]] property can be used to indicate entities that are the focus of the announcement. We now recommend using [[about]] only
/// for representing non-location entities (e.g. a [[Course]] or a [[RadioStation]]). For places, use [[announcementLocation]] and [[spatialCoverage]]. Consumers of this markup should be aware that the initial design encouraged the use of /about for locations too.
/// 
/// The basic content of [[SpecialAnnouncement]] is similar to that of an [RSS](https://en.wikipedia.org/wiki/RSS) or [Atom](https://en.wikipedia.org/wiki/Atom_(Web_standard)) feed. For publishers without such feeds, basic feed-like information can be shared by posting
/// [[SpecialAnnouncement]] updates in a page, e.g. using JSON-LD. For sites with Atom/RSS functionality, you can point to a feed
/// with the [[webFeed]] property. This can be a simple URL, or an inline [[DataFeed]] object, with [[encodingFormat]] providing
/// media type information e.g. "application/rss+xml" or "application/atom+xml".
struct SpecialAnnouncement;

/// A [[CompleteDataFeed]] is a [[DataFeed]] whose standard representation includes content for every item currently in the feed.
/// 
/// This is the equivalent of Atom's element as defined in Feed Paging and Archiving [RFC 5005](https://tools.ietf.org/html/rfc5005), For example (and as defined for Atom), when using data from a feed that represents a collection of items that varies over time (e.g. "Top Twenty Records") there is no need to have newer entries mixed in alongside older, obsolete entries. By marking this feed as a CompleteDataFeed, old entries can be safely discarded when the feed is refreshed, since we can assume the feed has provided descriptions for all current items.
struct CompleteDataFeed;

/// The manufacturer of the product.
struct manufacturer;

/// The base salary of the job or of an employee in an EmployeeRole.
struct baseSalary;

/// A WebSite is a set of related web pages and other items typically served from a single web domain and accessible via URLs.
struct WebSite;

/// Categories that represent an assessment of the risk of fetal injury due to a drug or pharmaceutical used as directed by the mother during pregnancy.
struct DrugPregnancyCategory;

/// A list of items of any sort&#x2014;for example, Top 10 Movies About Weathermen, or Top 100 Party Songs. Not to be confused with HTML lists, which are often used only for formatting.
struct ItemList;

/// A diet restricted to certain foods or preparations for cultural, religious, health or lifestyle reasons. 
struct RestrictedDiet;

/// OnlineEventAttendanceMode - an event that is primarily conducted online. 
struct OnlineEventAttendanceMode;

/// A taxi stand.
struct TaxiStand;

/// A CDCPMDRecord is a data structure representing a record in a CDC tabular data format
///       used for hospital data reporting. See [documentation](/docs/cdc-covid.html) for details, and the linked CDC materials for authoritative
///       definitions used as the source here.
///       
struct CDCPMDRecord;

/// The 75th percentile value.
struct percentile75;

/// Classes of agents or pathogens that transmit infectious diseases. Enumerated type.
struct InfectiousAgentClass;

/// Whether The rate of coinsurance expressed as a number between 0.0 and 1.0.
struct healthPlanCoinsuranceRate;

/// The basic containment relation between a place and one that contains it.
struct containedInPlace;

/// The header section of the page.
struct WPHeader;

/// A citation or reference to another creative work, such as another publication, web page, scholarly article, etc.
struct citation;

/// A vet's office.
struct VeterinaryCare;

/// An organizer of an Event.
struct organizer;

/// The product or service this support contact point is related to (such as product support for a particular product line). This can be a specific product or product line (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
struct productSupported;

/// The distance traveled per unit of fuel used; most commonly miles per gallon (mpg) or kilometers per liter (km/L).\n\n* Note 1: There are unfortunately no standard unit codes for miles per gallon or kilometers per liter. Use [[unitText]] to indicate the unit of measurement, e.g. mpg or km/L.\n* Note 2: There are two ways of indicating the fuel consumption, [[fuelConsumption]] (e.g. 8 liters per 100 km) and [[fuelEfficiency]] (e.g. 30 miles per gallon). They are reciprocal.\n* Note 3: Often, the absolute value is useful only when related to driving speed ("at 80 km/h") or usage pattern ("city traffic"). You can use [[valueReference]] to link the value for the fuel economy to another value.
struct fuelEfficiency;

/// A process of care used in either a diagnostic, therapeutic, preventive or palliative capacity that relies on invasive (surgical), non-invasive, or other techniques.
struct MedicalProcedure;

/// Other anatomical structures to which this structure is connected.
struct connectedTo;

/// The payment method(s) accepted by seller for this offer.
struct acceptedPaymentMethod;

/// The legal value of this legislation file. The same legislation can be written in multiple files with different legal values. Typically a digitally signed PDF have a "stronger" legal value than the HTML file of the same act.
struct legislationLegalValue;

/// A predefined value from OfferItemCondition or a textual description of the condition of the product or service, or the products or services included in the offer.
struct itemCondition;

/// A type of boarding policy used by an airline.
struct BoardingPolicyType;

/// Indicates the approximate radius of a GeoCircle (metres unless indicated otherwise via Distance notation).
struct geoRadius;

/// Play mode: SinglePlayer. Which is played by a lone player.
struct SinglePlayer;

/// Information about the engine of the vehicle. A vehicle can have multiple engines represented by multiple engine specification entities.
struct EngineSpecification;

/// A preschool.
struct Preschool;

/// The basic containment relation between a place and another that it contains.
struct containsPlace;

/// An additional offer that can only be obtained in combination with the first base offer (e.g. supplements and extensions that are available for a surcharge).
struct addOn;

/// A sign or symptom of this condition. Signs are objective or physically observable manifestations of the medical condition while symptoms are the subjective experience of the medical condition.
struct signOrSymptom;

/// A blog post intended to provide a rolling textual coverage of an ongoing event through continuous updates.
struct LiveBlogPosting;

/// A medical procedure intended primarily for therapeutic purposes, aimed at improving a health condition.
struct TherapeuticProcedure;

/// A flag to signal that the [[Place]] is open to public visitors.  If this property is omitted there is no assumed default boolean value
struct publicAccess;

/// An agent joins an event/group with participants/friends at a location.\n\nRelated actions:\n\n* [[RegisterAction]]: Unlike RegisterAction, JoinAction refers to joining a group/team of people.\n* [[SubscribeAction]]: Unlike SubscribeAction, JoinAction does not imply that you'll be receiving updates.\n* [[FollowAction]]: Unlike FollowAction, JoinAction does not imply that you'll be polling for updates.
struct JoinAction;

/// A sub property of instrument. The diet used in this action.
struct exerciseRelatedDiet;

/// Text of an utterances (spoken words, lyrics etc.) that occurs at a certain section of a media object, represented as a [[HyperTocEntry]].
struct utterances;

/// Text of a notice appropriate for describing the copyright aspects of this Creative Work, ideally indicating the owner of the copyright for the Work.
struct copyrightNotice;

/// A medical trial is a type of medical study that uses scientific process used to compare the safety and efficacy of medical therapies or medical procedures. In general, medical trials are controlled and subjects are allocated at random to the different treatment and/or control groups.
struct MedicalTrial;

/// The CO2 emissions in g/km. When used in combination with a QuantitativeValue, put "g/km" into the unitText property of that value, since there is no UN/CEFACT Common Code for "g/km".
struct emissionsCO2;

/// A medical procedure intended primarily for diagnostic, as opposed to therapeutic, purposes.
struct DiagnosticProcedure;

/// A box is the area enclosed by the rectangle formed by two points. The first point is the lower corner, the second point is the upper corner. A box is expressed as two points separated by a space character.
struct box;

/// Days of the week when the merchant typically operates, indicated via opening hours markup.
struct businessDays;

/// A step in the instructions for how to achieve a result. It is an ordered list with HowToDirection and/or HowToTip items.
struct HowToStep;

/// Represents additional information about a relationship or property. For example a Role can be used to say that a 'member' role linking some SportsTeam to a player occurred during a particular time period. Or that a Person's 'actor' role in a Movie was for some particular characterName. Such properties can be attached to a Role entity, which is then associated with the main entities using ordinary properties like 'member' or 'actor'.\n\nSee also [blog post](http://blog.schema.org/2014/06/introducing-role.html).
struct Role;

/// Categories of medical devices, organized by the purpose or intended use of the device.
struct MedicalDevicePurpose;

/// Web page type: Image gallery page.
struct ImageGallery;

/// Alumni of an organization.
struct alumni;

/// Indicates the timezone for which the time(s) indicated in the [[Schedule]] are given. The value provided should be among those listed in the IANA Time Zone Database.
struct scheduleTimezone;

/// An accommodation is a place that can accommodate human beings, e.g. a hotel room, a camping pitch, or a meeting room. Many accommodations are for overnight stays, but this is not a mandatory requirement.
/// For more specific types of accommodations not defined in schema.org, one can use additionalType with external vocabularies.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct Accommodation;

/// The item that is being reviewed/rated.
struct itemReviewed;

/// A locksmith.
struct Locksmith;

/// A body of water, such as a sea, ocean, or lake.
struct BodyOfWater;

/// An actor, e.g. in tv, radio, movie, video games etc. Actors can be associated with individual items or with a series, episode, clip.
struct actors;

/// Data type: PronounceableText.
struct PronounceableText;

/// A venue map (e.g. for malls, auditoriums, museums, etc.).
struct VenueMap;

/// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
struct step;

/// The person, organization, contact point, or audience that has been granted this permission.
struct grantee;

/// The organization owning or operating the broadcast service.
struct broadcaster;

/// The end of the availability of the product or service included in the offer.
struct availabilityEnds;

/// The highest value allowed in this rating system. If bestRating is omitted, 5 is assumed.
struct bestRating;

/// A video game is an electronic game that involves human interaction with a user interface to generate visual feedback on a video device.
struct VideoGame;

/// Content about contagion mechanisms and contagiousness information over the topic.
struct ContagiousnessHealthAspect;

/// Organization: Non-governmental Organization.
struct NGO;

/// A sub property of location. The course where this action was taken.
struct course;

/// A value indicating which roadwheels will receive torque.
struct DriveWheelConfigurationValue;

/// The day of the week, e.g. used to specify to which day the opening hours of an OpeningHoursSpecification refer.
/// 
/// Originally, URLs from [GoodRelations](http://purl.org/goodrelations/v1) were used (for [[Monday]], [[Tuesday]], [[Wednesday]], [[Thursday]], [[Friday]], [[Saturday]], [[Sunday]] plus a special entry for [[PublicHolidays]]); these have now been integrated directly into schema.org.
///       
struct DayOfWeek;

/// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection (originally misnamed 'steps'; 'step' is preferred).
struct steps;

/// Description of benefits associated with the job.
struct jobBenefits;

/// numvent - MECHANICAL VENTILATORS: Total number of ventilators available.
struct cvdNumVent;

/// A reservation to dine at a food-related business.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations.
struct FoodEstablishmentReservation;

/// An item being offered (or demanded). The transactional nature of the offer or demand is documented using [[businessFunction]], e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
struct itemOffered;

/// A service provided by an organization, e.g. delivery service, print services, etc.
struct Service;

/// The number of passengers that can be seated in the vehicle, both in terms of the physical space available, and in terms of limitations set by law.\n\nTypical unit code(s): C62 for persons.
struct vehicleSeatingCapacity;

/// The page on which the work starts; for example "135" or "xiii".
struct pageStart;

/// Enumerates different price types, for example list price, invoice price, and sale price.
struct PriceTypeEnumeration;

/// Any medical test, typically performed for diagnostic purposes.
struct MedicalTest;

/// A summary of how users have interacted with this CreativeWork. In most cases, authors will use a subtype to specify the specific type of interaction.
struct InteractionCounter;

/// The 10th percentile value.
struct percentile10;

/// Content about common misconceptions and myths that are related to a topic.
struct MisconceptionsHealthAspect;

/// The act of expressing a preference from a fixed/finite/structured set of choices/options.
struct VoteAction;

/// Countries for which the application is not supported. You can also provide the two-letter ISO 3166-1 alpha-2 country code.
struct countriesNotSupported;

/// The home team in a sports event.
struct homeTeam;

/// The typical expected age range, e.g. '7-9', '11-'.
struct typicalAgeRange;

/// Event type: Exhibition event, e.g. at a museum, library, archive, tradeshow, ...
struct ExhibitionEvent;

/// The status of payment; whether the invoice has been paid or not.
struct paymentStatus;

/// Processor architecture required to run the application (e.g. IA64).
struct processorRequirements;

/// A home goods store.
struct HomeGoodsStore;

/// Indicates a dietary restriction or guideline for which this recipe or menu item is suitable, e.g. diabetic, halal etc.
struct suitableForDiet;

/// Nutritional information specific to the dietary plan. May include dietary recommendations on what foods to avoid, what foods to consume, and specific alterations/deviations from the USDA or other regulatory body's approved dietary guidelines.
struct dietFeatures;

/// The place where the person died.
struct deathPlace;

/// Statement on diversity policy by an [[Organization]] e.g. a [[NewsMediaOrganization]]. For a [[NewsMediaOrganization]], a statement describing the newsroom’s diversity policy on both staffing and sources, typically providing staffing data.
struct diversityPolicy;

/// Indicates that a legislation is currently not in force.
struct NotInForce;

/// Minimal age recommended for viewing content.
struct suggestedMinAge;

/// Indicates a textual identifier for a ProductGroup.
struct productGroupID;

/// The publisher of the creative work.
struct publisher;

/// The type of the medical article, taken from the US NLM MeSH publication type catalog. See also [MeSH documentation](http://www.nlm.nih.gov/mesh/pubtypes.html).
struct publicationType;

/// A crematorium.
struct Crematorium;

/// The act of applying an object to its intended purpose.
struct UseAction;

/// The delivery of a parcel either via the postal service or a commercial service.
struct ParcelDelivery;

/// A sub property of result. The review that resulted in the performing of the action.
struct resultReview;

/// A diet appropriate for people with lactose intolerance.
struct LowLactoseDiet;

/// Recommended intake of this supplement for a given population as defined by a specific recommending authority.
struct recommendedIntake;

/// A specific branch of medical science that deals with the study and treatment of rheumatic, autoimmune or joint diseases.
struct Rheumatologic;

/// A set of products (either [[ProductGroup]]s or specific variants) that are listed together e.g. in an [[Offer]].
struct ProductCollection;

/// A sub property of location. The final location of the object or the agent after the action.
struct toLocation;

/// Type of employment (e.g. full-time, part-time, contract, temporary, seasonal, internship).
struct employmentType;

/// A type of bed. This is used for indicating the bed or beds available in an accommodation.
struct BedType;

/// The act of expressing a negative sentiment about the object. An agent dislikes an object (a proposition, topic or theme) with participants.
struct DislikeAction;

/// How often one should engage in the activity.
struct activityFrequency;

/// The act of responding instinctively and emotionally to an object, expressing a sentiment.
struct ReactAction;

/// Nonprofit501c20: Non-profit type referring to Group Legal Services Plan Organizations.
struct Nonprofit501c20;

/// A specific branch of medical science that pertains to the study of the respiratory system and its respective disease states.
struct Pulmonary;

/// The computer programming language.
struct programmingLanguage;

/// WebContent is a type representing all [[WebPage]], [[WebSite]] and [[WebPageElement]] content. It is sometimes the case that detailed distinctions between Web pages, sites and their parts is not always important or obvious. The  [[WebContent]] type makes it easier to describe Web-addressable content without requiring such distinctions to always be stated. (The intent is that the existing types [[WebPage]], [[WebSite]] and [[WebPageElement]] will eventually be declared as subtypes of [[WebContent]].)
///   
struct WebContent;

/// The only way you get the money back in the event of default is the security. Recourse is where you still have the opportunity to go back to the borrower for the rest of the money.
struct recourseLoan;

/// A publication event associated with the item.
struct publication;

/// Responsibilities associated with this role or Occupation.
struct responsibilities;

/// The result produced in the action. e.g. John wrote *a book*.
struct result;

/// The currency in which the monetary amount is expressed.\n\nUse standard formats: [ISO 4217 currency format](http://en.wikipedia.org/wiki/ISO_4217) e.g. "USD"; [Ticker symbol](https://en.wikipedia.org/wiki/List_of_cryptocurrencies) for cryptocurrencies e.g. "BTC"; well known names for [Local Exchange Tradings Systems](https://en.wikipedia.org/wiki/Local_exchange_trading_system) (LETS) and other currency types e.g. "Ithaca HOUR".
struct currency;

/// A parents of the person.
struct parents;

/// A music video file.
struct MusicVideoObject;

/// The status of a medical study. Enumerated type.
struct MedicalStudyStatus;

/// The tangible thing generated by the service, e.g. a passport, permit, etc.
struct produces;

/// The number of attendee places for an event that remain unallocated.
struct remainingAttendeeCapacity;

/// A description of the variant cover
///     	for the issue, if the issue is a variant printing. For example, "Bryan Hitch
///     	Variant Cover" or "2nd Printing Variant".
struct variantCover;

/// The [GTIN-8](http://apps.gs1.org/GDD/glossary/Pages/GTIN-8.aspx) code of the product, or the product to which the offer refers. This code is also known as EAN/UCC-8 or 8-digit EAN. See [GS1 GTIN Summary](http://www.gs1.org/barcodes/technical/idkeys/gtin) for more details.
struct gtin8;

/// A music recording (track)&#x2014;usually a single song. If an ItemList is given, the list should contain items of type MusicRecording.
struct track;

/// A prion is an infectious agent composed of protein in a misfolded form.
struct Prion;

/// numc19ofmechventpats - ED/OVERFLOW and VENTILATED: Patients with suspected or confirmed COVID-19 who are in the ED or any overflow location awaiting an inpatient bed and on a mechanical ventilator.
struct cvdNumC19OFMechVentPats;

/// Event type: Children's event.
struct ChildrensEvent;

/// The class of infectious agent (bacteria, prion, etc.) that causes the disease.
struct infectiousAgentClass;

/// Indicates the current disposition of the Action.
struct actionStatus;

/// Name of the County of the NHSN facility that this data record applies to. Use [[cvdFacilityId]] to identify the facility. To provide other details, [[healthcareReportingData]] can be used on a [[Hospital]] entry.
struct cvdFacilityCounty;

/// Pregnancy category of this drug.
struct pregnancyCategory;

/// The time when the live blog will begin covering the Event. Note that coverage may begin before the Event's start time. The LiveBlogPosting may also be created before coverage begins.
struct coverageStartTime;

/// Physical activity that is engaged in to improve joint and muscle flexibility.
struct Flexibility;

/// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
struct provider;

/// A sub property of location. The original location of the object or the agent before the action.
struct fromLocation;

/// Book format: Hardcover.
struct Hardcover;

/// A child of the person.
struct children;

/// Quiz: A test of knowledge, skills and abilities.
struct Quiz;

/// A sub property of participant. The real estate agent involved in the action.
struct realEstateAgent;

/// The URL at which a reply may be posted to the specified UserComment.
struct replyToUrl;

/// The stage of the condition, if applicable.
struct stage;

/// The serial number or any alphanumeric identifier of a particular product. When attached to an offer, it is a shortcut for the serial number of the product included in the offer.
struct serialNumber;

/// A person who founded this organization.
struct founders;

/// The weight of the product or person.
struct weight;

/// A designation by the US FDA signifying that animal reproduction studies have shown an adverse effect on the fetus and there are no adequate and well-controlled studies in humans, but potential benefits may warrant use of the drug in pregnant women despite potential risks.
struct FDAcategoryC;

/// The number of seasons in this series.
struct numberOfSeasons;

/// A specific branch of medical science that pertains to study of anesthetics and their application.
struct Anesthesia;

/// The ACRISS Car Classification Code is a code used by many car rental companies, for classifying vehicles. ACRISS stands for Association of Car Rental Industry Systems and Standards.
struct acrissCode;

/// The geographic area where a permit or similar thing is valid.
struct validIn;

/// ParentalSupport: this is a benefit for parental support.
struct ParentalSupport;

/// An ice cream shop.
struct IceCreamShop;

/// A range of of services that will be provided to a customer free of charge in case of a defect or malfunction of a product.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#Labor-BringIn\n* http://purl.org/goodrelations/v1#PartsAndLabor-BringIn\n* http://purl.org/goodrelations/v1#PartsAndLabor-PickUp
///       
struct WarrantyScope;

/// UKNonprofitType: Non-profit organization type originating from the United Kingdom.
struct UKNonprofitType;

/// The locality in which the street address is, and which is in the region. For example, Mountain View.
struct addressLocality;

/// The location in which the status applies.
struct applicableLocation;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserLikes;

/// The startTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to start. For actions that span a period of time, when the action was performed. e.g. John wrote a book from *January* to December. For media, including audio and video, it's the time offset of the start of a clip within a larger file.\n\nNote that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
struct startTime;

/// The length of time it takes to perform instructions or a direction (not including time to prepare the supplies), in [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601).
struct performTime;

/// A FullRefund ...
struct FullRefund;

/// The date/time at which the message has been read by the recipient if a single recipient exists.
struct dateRead;

/// A structured value representing a price or price range. Typically, only the subclasses of this type are used for markup. It is recommended to use [[MonetaryAmount]] to describe independent amounts of money such as a salary, credit card limits, etc.
struct PriceSpecification;

/// Pathogenic bacteria that cause bacterial infection.
struct Bacteria;

/// Indicates that the item is available only at physical locations.
struct InStoreOnly;

/// Link to the repository where the un-compiled, human readable code and related code is located (SVN, github, CodePlex).
struct codeRepository;

/// An agent approves/certifies/likes/supports/sanction an object.
struct EndorseAction;

/// The depth of the item.
struct depth;

/// Position of the season within an ordered group of seasons.
struct seasonNumber;

/// A radio station.
struct RadioStation;

/// Indicates whether the book is an abridged edition.
struct abridged;

/// A post office.
struct PostOffice;

/// Nonprofit527: Non-profit type referring to Political organizations.
struct Nonprofit527;

/// A program offered by an institution which determines the learning progress to achieve an outcome, usually a credential like a degree or certificate. This would define a discrete set of opportunities (e.g., job, courses) that together constitute a program with a clear start, end, set of requirements, and transition to a new occupational opportunity (e.g., a job), or sometimes a higher educational opportunity (e.g., an advanced degree).
struct EducationalOccupationalProgram;

/// A taxi.
struct Taxi;

/// A sub property of participant. The sports team that participated on this action.
struct sportsTeam;

/// Content features of the resource, such as accessible media, alternatives and supported enhancements for accessibility ([WebSchemas wiki lists possible values](http://www.w3.org/wiki/WebSchemas/Accessibility)).
struct accessibilityFeature;

/// A bowling alley.
struct BowlingAlley;

/// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
struct category;

/// collectiondate - Date for which patient counts are reported.
struct cvdCollectionDate;

/// A publication in any medium issued in successive parts bearing numerical or chronological designations and intended, such as a magazine, scholarly journal, or newspaper to continue indefinitely.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
struct Periodical;

/// The RxCUI drug identifier from RXNORM.
struct rxcui;

/// For failed actions, more information on the cause of the failure.
struct error;

/// Game server status: OnlineFull. Server is online but unavailable. The maximum number of players has reached.
struct OnlineFull;

/// A bookstore.
struct BookStore;

/// A playground.
struct Playground;

/// A collection of music albums.
struct albums;

/// A payment method is a standardized procedure for transferring the monetary amount for a purchase. Payment methods are characterized by the legal and technical structures used, and by the organization or group carrying out the transaction.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#ByBankTransferInAdvance\n* http://purl.org/goodrelations/v1#ByInvoice\n* http://purl.org/goodrelations/v1#Cash\n* http://purl.org/goodrelations/v1#CheckInAdvance\n* http://purl.org/goodrelations/v1#COD\n* http://purl.org/goodrelations/v1#DirectDebit\n* http://purl.org/goodrelations/v1#GoogleCheckout\n* http://purl.org/goodrelations/v1#PayPal\n* http://purl.org/goodrelations/v1#PaySwarm
///         
struct PaymentMethod;

/// The format of the book.
struct bookFormat;

/// Not yet recruiting.
struct NotYetRecruiting;

/// Genitourinary system function assessment with clinical examination.
struct Genitourinary;

/// A food or drink item contained in a menu or menu section.
struct hasMenuItem;

/// Guidelines about quarantine rules, e.g. in the context of a pandemic.
struct quarantineGuidelines;

/// A volcano, like Fuji san.
struct Volcano;

/// The number of items in an ItemList. Note that some descriptions might not fully describe all items in a list (e.g., multi-page pagination); in such cases, the numberOfItems would be for the entire list.
struct numberOfItems;

/// Dietetic and nutrition as a medical speciality.
struct DietNutrition;

/// The event is taking place or has taken place on the startDate as scheduled. Use of this value is optional, as it is assumed by default.
struct EventScheduled;

/// A posting to a discussion forum.
struct DiscussionForumPosting;

/// An update to the LiveBlog.
struct liveBlogUpdate;

/// A short explanation (e.g. one to two sentences) providing background context and other information that led to the conclusion expressed in the rating. This is particularly applicable to ratings associated with "fact check" markup using [[ClaimReview]].
struct ratingExplanation;

/// Articles may belong to one or more 'sections' in a magazine or newspaper, such as Sports, Lifestyle, etc.
struct articleSection;

/// The individual who draws the primary narrative artwork.
struct penciler;

/// Strength of the guideline's recommendation (e.g. 'class I').
struct recommendationStrength;

/// A short TV or radio program or a segment/part of a program.
struct Clip;

/// Eye or ophtalmological function assessment with clinical examination.
struct Eye;

/// A direction indicating a single action to do in the instructions for how to achieve a result.
struct HowToDirection;

/// A reservation for a rental car.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations.
struct RentalCarReservation;

/// Completed.
struct Completed;

/// A subway station.
struct SubwayStation;

/// The number or type of airbags in the vehicle.
struct numberOfAirbags;

/// The algorithm or rules to follow to compute the score.
struct algorithm;

/// Specifies the allowed range for number of characters in a literal value.
struct valueMaxLength;

/// A sub-grouping of steps in the instructions for how to achieve a result (e.g. steps for making a pie crust within a pie recipe).
struct HowToSection;

/// A store that sells mobile phones and related accessories.
struct MobilePhoneStore;

/// A list of possible levels for the legal validity of a legislation.
struct LegalValueLevel;

/// The time admission will commence.
struct doorTime;

/// Link to prescribing information for the drug.
struct prescribingInfo;

/// A trip on a commercial bus line.
struct BusTrip;

/// The fax number.
struct faxNumber;

/// 'vendor' is an earlier term for 'seller'.
struct vendor;

/// The type(s) of customers for which the given offer is valid.
struct eligibleCustomerType;

/// The currency (coded using [ISO 4217](http://en.wikipedia.org/wiki/ISO_4217) ) used for the main salary information in this job posting or for this employee.
struct salaryCurrency;

/// A pointer to another product (or multiple products) for which this product is a consumable.
struct isConsumableFor;

/// A description of the types of physical activity associated with the job. Defined terms such as those in O*net may be used, but note that there is no way to specify the level of ability as well as its nature when using a defined term.
struct physicalRequirement;

/// The warranty promise(s) included in the offer.
struct warrantyPromise;

/// The station where the train trip ends.
struct arrivalStation;

/// The date that payment is due.
struct paymentDue;

/// An ItemList ordered with higher values listed first.
struct ItemListOrderDescending;

/// An [EIDR](https://eidr.org/) (Entertainment Identifier Registry) [[identifier]] representing a specific edit / edition for a work of film or television.
/// 
/// For example, the motion picture known as "Ghostbusters" whose [[titleEIDR]] is "10.5240/7EC7-228A-510A-053E-CBB8-J", has several edits e.g. "10.5240/1F2A-E1C5-680A-14C6-E76B-I" and "10.5240/8A35-3BEE-6497-5D12-9E4F-3".
/// 
/// Since schema.org types like [[Movie]] and [[TVEpisode]] can be used for both works and their multiple expressions, it is possible to use [[titleEIDR]] alone (for a general description), or alongside [[editEIDR]] for a more edit-specific description.
struct editEIDR;

/// Indicates the first known occurence of a [[Claim]] in some [[CreativeWork]].
struct firstAppearance;

/// A relationship between two organizations where the first includes the second, e.g., as a subsidiary. See also: the more specific 'department' property.
struct subOrganization;

/// Indicates a document for which the text is conclusively what the law says and is legally binding. (e.g. The digitally signed version of an Official Journal.)
///   Something "Definitive" is considered to be also [[AuthoritativeLegalValue]].
struct DefinitiveLegalValue;

/// Beach.
struct Beach;

/// A train station.
struct TrainStation;

/// A sub property of recipient. The recipient who was directly sent the message.
struct toRecipient;

/// A School District is an administrative area for the administration of schools.
struct SchoolDistrict;

/// Thumbnail image for an image or video.
struct thumbnail;

/// A PublicationEvent corresponds indifferently to the event of publication for a CreativeWork of any type e.g. a broadcast event, an on-demand event, a book/journal publication via a variety of delivery media.
struct PublicationEvent;

/// The anatomical or organ system that this structure is part of.
struct partOfSystem;

/// A permit issued by a government agency.
struct GovernmentPermit;

/// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
struct disambiguatingDescription;

/// The latitude of a location. For example ```37.42242``` ([WGS 84](https://en.wikipedia.org/wiki/World_Geodetic_System)).
struct latitude;

/// When a single product is associated with multiple offers (for example, the same pair of shoes is offered by different merchants), then AggregateOffer can be used.\n\nNote: AggregateOffers are normally expected to associate multiple offers that all share the same defined [[businessFunction]] value, or default to http://purl.org/goodrelations/v1#Sell if businessFunction is not explicitly defined.
struct AggregateOffer;

/// A material used as a surface in some artwork, e.g. Canvas, Paper, Wood, Board, etc.
struct surface;

/// A product taken by mouth that contains a dietary ingredient intended to supplement the diet. Dietary ingredients may include vitamins, minerals, herbs or other botanicals, amino acids, and substances such as enzymes, organ tissues, glandulars and metabolites.
struct DietarySupplement;

/// 'carrier' is an out-dated term indicating the 'provider' for parcel delivery and flights.
struct carrier;

/// The cuisine of the restaurant.
struct servesCuisine;

/// A Childcare center.
struct ChildCare;

/// A monetary grant.
struct MonetaryGrant;

/// A set of requirements that a must be fulfilled in order to perform an Action. If more than one value is specied, fulfilling one set of requirements will allow the Action to be performed.
struct actionAccessibilityRequirement;

/// A step in making the recipe, in the form of a single item (document, video, etc.) or an ordered list with HowToStep and/or HowToSection items.
struct recipeInstructions;

/// The release date of a vehicle model (often used to differentiate versions of the same make and model).
struct modelDate;

/// Defines the day(s) of the month on which a recurring [[Event]] takes place. Specified as an [[Integer]] between 1-31.
struct byMonthDay;

/// The act of generating a comment about a subject.
struct CommentAction;

/// A structured value representing exchange rate.
struct ExchangeRateSpecification;

/// A dry-cleaning business.
struct DryCleaningOrLaundry;

/// A description of an educational course which may be offered as distinct instances at which take place at different times or take place at different locations, or be offered through different media or modes of study. An educational course is a sequence of one or more educational events and/or creative works which aims to build knowledge, competence or ability of learners.
struct Course;

/// The day of the week between Wednesday and Friday.
struct Thursday;

/// Length of the lease for some [[Accommodation]], either particular to some [[Offer]] or in some cases intrinsic to the property.
struct leaseLength;

/// A grant, typically financial or otherwise quantifiable, of resources. Typically a [[funder]] sponsors some [[MonetaryAmount]] to an [[Organization]] or [[Person]],
///     sometimes not necessarily via a dedicated or long-lived [[Project]], resulting in one or more outputs, or [[fundedItem]]s. For financial sponsorship, indicate the [[funder]] of a [[MonetaryGrant]]. For non-financial support, indicate [[sponsor]] of [[Grant]]s of resources (e.g. office space).
/// 
/// Grants support  activities directed towards some agreed collective goals, often but not always organized as [[Project]]s. Long-lived projects are sometimes sponsored by a variety of grants over time, but it is also common for a project to be associated with a single grant.
/// 
/// The amount of a [[Grant]] is represented using [[amount]] as a [[MonetaryAmount]].
///     
struct Grant;

/// New entry added as the package passes through each leg of its journey (from shipment to final delivery).
struct deliveryStatus;

/// The condition, complication, or symptom whose risk is being estimated.
struct estimatesRiskOf;

/// Assets required to secure loan or credit repayments. It may take form of third party pledge, goods, financial instruments (cash, securities, etc.)
struct requiredCollateral;

/// A description of the workup, testing, and other preparations required before implanting this device.
struct preOp;

/// The BroadcastService offered on this channel.
struct providesBroadcastService;

/// The number of comments this CreativeWork (e.g. Article, Question or Answer) has received. This is most applicable to works published in Web sites with commenting system; additional comments may exist elsewhere.
struct commentCount;

/// Indicates that the vehicle meets the respective emission standard.
struct meetsEmissionStandard;

/// This ordering relation for qualitative values indicates that the subject is not equal to the object.
struct nonEqual;

/// Indicates that the item is out of stock.
struct OutOfStock;

/// Web applications.
struct WebApplication;

/// Indicates an OfferCatalog listing for this Organization, Person, or Service.
struct hasOfferCatalog;

/// A schedule defines a repeating time period used to describe a regularly occurring [[Event]]. At a minimum a schedule will specify [[repeatFrequency]] which describes the interval between occurences of the event. Additional information can be provided to specify the schedule more precisely.
///       This includes identifying the day(s) of the week or month when the recurring event will take place, in addition to its start and end time. Schedules may also
///       have start and end dates to indicate when they are active, e.g. to define a limited calendar of events.
struct Schedule;

/// The number of payments contractually required at origination to repay the loan. For monthly paying loans this is the number of months from the contractual first payment date to the maturity date.
struct numberOfLoanPayments;

/// A [dateline](https://en.wikipedia.org/wiki/Dateline) is a brief piece of text included in news articles that describes where and when the story was written or filed though the date is often omitted. Sometimes only a placename is provided.
/// 
/// Structured representations of dateline-related information can also be expressed more explicitly using [[locationCreated]] (which represents where a work was created e.g. where a news report was written).  For location depicted or described in the content, use [[contentLocation]].
/// 
/// Dateline summaries are oriented more towards human readers than towards automated processing, and can vary substantially. Some examples: "BEIRUT, Lebanon, June 2.", "Paris, France", "December 19, 2017 11:43AM Reporting from Washington", "Beijing/Moscow", "QUEZON CITY, Philippines".
///       
struct dateline;

/// If this NewsArticle appears in print, this field indicates the name of the page on which the article is found. Please note that this field is intended for the exact page name (e.g. A5, B18).
struct printPage;

/// A unique instance of a BroadcastService on a CableOrSatelliteService lineup.
struct BroadcastChannel;

/// Smaller compositions included in this work (e.g. a movement in a symphony).
struct includedComposition;

/// Requirements for taking the Course. May be completion of another [[Course]] or a textual description like "permission of instructor". Requirements may be a pre-requisite competency, referenced using [[AlignmentObject]].
struct coursePrerequisites;

/// A broadcast service associated with the publication event.
struct publishedOn;

/// Additional menu item(s) such as a side dish of salad or side order of fries that can be added to this menu item. Additionally it can be a menu section containing allowed add-on menu items for this menu item.
struct menuAddOn;

/// A furniture store.
struct FurnitureStore;

/// The frequency used for over-the-air broadcasts. Numeric values or simple ranges e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
struct broadcastFrequency;

/// Storage requirements (free space required).
struct storageRequirements;

/// A LegalService is a business that provides legally-oriented services, advice and representation, e.g. law firms.\n\nAs a [[LocalBusiness]] it can be described as a [[provider]] of one or more [[Service]]\(s).
struct LegalService;

/// The day of the week between Sunday and Tuesday.
struct Monday;

/// Media type, typically MIME format (see [IANA site](http://www.iana.org/assignments/media-types/media-types.xhtml)) of the content e.g. application/zip of a SoftwareApplication binary. In cases where a CreativeWork has several media type representations, 'encoding' can be used to indicate each MediaObject alongside particular fileFormat information. Unregistered or niche file formats can be indicated instead via the most appropriate URL, e.g. defining Web page or a Wikipedia entry.
struct fileFormat;

/// Represents EU Energy Efficiency Class E as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryE;

/// CDFormat.
struct CDFormat;

/// The type of procedure, for example Surgical, Noninvasive, or Percutaneous.
struct procedureType;

/// A reservation for a taxi.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
struct TaxiReservation;

/// Of a [[Person]], and less typically of an [[Organization]], to indicate a known language. We do not distinguish skill levels or reading/writing/speaking/signing here. Use language codes from the [IETF BCP 47 standard](http://tools.ietf.org/html/bcp47).
struct knowsLanguage;

/// A pattern that something has, for example 'polka dot', 'striped', 'Canadian flag'. Values are typically expressed as text, although links to controlled value schemes are also supported.
struct pattern;

/// An indication for preventing an underlying condition, symptom, etc.
struct PreventionIndication;

/// Nonprofit501c15: Non-profit type referring to Mutual Insurance Companies or Associations.
struct Nonprofit501c15;

/// The time of day the program normally runs. For example, "evenings".
struct timeOfDay;

/// The maximum physical attendee capacity of an [[Event]] whose [[eventAttendanceMode]] is [[OnlineEventAttendanceMode]] (or the online aspects, in the case of a [[MixedEventAttendanceMode]]). 
struct maximumVirtualAttendeeCapacity;

/// Indicates sections of a Web page that are particularly 'speakable' in the sense of being highlighted as being especially appropriate for text-to-speech conversion. Other sections of a page may also be usefully spoken in particular circumstances; the 'speakable' property serves to indicate the parts most likely to be generally useful for speech.
/// 
/// The *speakable* property can be repeated an arbitrary number of times, with three kinds of possible 'content-locator' values:
/// 
/// 1.) *id-value* URL references - uses *id-value* of an element in the page being annotated. The simplest use of *speakable* has (potentially relative) URL values, referencing identified sections of the document concerned.
/// 
/// 2.) CSS Selectors - addresses content in the annotated page, eg. via class attribute. Use the [[cssSelector]] property.
/// 
/// 3.)  XPaths - addresses content via XPaths (assuming an XML view of the content). Use the [[xpath]] property.
/// 
/// 
/// For more sophisticated markup of speakable sections beyond simple ID references, either CSS selectors or XPath expressions to pick out document section(s) as speakable. For this
/// we define a supporting type, [[SpeakableSpecification]]  which is defined to be a possible value of the *speakable* property.
///          
struct speakable;

/// A store that sells reading glasses and similar devices for improving vision.
struct Optician;

/// The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a product or service, or the product to which the offer refers.
struct sku;

/// A floorplan of some [[Accommodation]].
struct accommodationFloorPlan;

/// The boolean value false.
struct False;

/// A specific branch of medical science that pertains to diagnosis and treatment of disorders of blood and blood producing organs.
struct Hematologic;

/// numicubeds - ICU BEDS: Total number of staffed inpatient intensive care unit (ICU) beds.
struct cvdNumICUBeds;

/// A sub property of instrument. The method of delivery.
struct deliveryMethod;

/// The total number of individuals that may attend an event or venue.
struct maximumAttendeeCapacity;

/// The series to which this episode or season belongs.
struct partOfSeries;

/// A [[DefinedTermSet]] that contains this term.
struct inDefinedTermSet;

/// The earliest someone may check into a lodging establishment.
struct checkinTime;

/// A sub property of instrument. An object used (but not consumed) when performing instructions or a direction.
struct tool;

/// A media object representing the circumstances while performing this direction.
struct duringMedia;

/// A parking map.
struct ParkingMap;

/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that covers it. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoCoveredBy;

/// Aquarium.
struct Aquarium;

/// A house is a building or structure that has the ability to be occupied for habitation by humans or other creatures (Source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/House">http://en.wikipedia.org/wiki/House</a>).
struct House;

/// Specifies whether the applicable value-added tax (VAT) is included in the price specification or not.
struct valueAddedTaxIncluded;

/// A Report generated by governmental or non-governmental organization.
struct Report;

/// A stage of a medical condition, such as 'Stage IIIa'.
struct MedicalConditionStage;

/// This links to a node or nodes indicating the exact quantity of the products included in  an [[Offer]] or [[ProductCollection]].
struct includesObject;

/// The individual reservations included in the package. Typically a repeated property.
struct subReservation;

/// A terminal for boats, ships, and other water vessels.
struct BoatTerminal;

/// The act of ingesting information/resources/food.
struct ConsumeAction;

/// A designation by the US FDA signifying that studies in animals or humans have demonstrated fetal abnormalities and/or there is positive evidence of human fetal risk based on adverse reaction data from investigational or marketing experience, and the risks involved in use of the drug in pregnant women clearly outweigh potential benefits.
struct FDAcategoryX;

/// The annual rate that is charged for borrowing (or made by investing), expressed as a single percentage number that represents the actual yearly cost of funds over the term of a loan. This includes any fees or additional costs associated with the transaction.
struct annualPercentageRate;

/// Qualification, candidature, degree, application that Thesis supports.
struct inSupportOf;

/// The episode to which this clip belongs.
struct partOfEpisode;

/// The act of providing an object under an agreement that it will be returned at a later date. Reciprocal of BorrowAction.\n\nRelated actions:\n\n* [[BorrowAction]]: Reciprocal of LendAction.
struct LendAction;

/// Nonprofit501c7: Non-profit type referring to Social and Recreational Clubs.
struct Nonprofit501c7;

/// A diet conforming to Islamic dietary practices.
struct HalalDiet;

/// A CreativeWork or Event about this Thing.
struct subjectOf;

/// Format of this release (the type of recording media used, ie. compact disc, digital media, LP, etc.).
struct musicReleaseFormat;

/// The duration of the loan or credit agreement.
struct loanTerm;

/// Media type typically expressed using a MIME format (see [IANA site](http://www.iana.org/assignments/media-types/media-types.xhtml) and [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)) e.g. application/zip for a SoftwareApplication binary, audio/mpeg for .mp3 etc.).
/// 
/// In cases where a [[CreativeWork]] has several media type representations, [[encoding]] can be used to indicate each [[MediaObject]] alongside particular [[encodingFormat]] information.
/// 
/// Unregistered or niche encoding and file formats can be indicated instead via the most appropriate URL, e.g. defining Web page or a Wikipedia/Wikidata entry.
struct encodingFormat;

/// How often one should break from the activity.
struct restPeriods;

/// Additional details to capture the origin of the cost data. For example, 'Medicare Part B'.
struct costOrigin;

/// The post office box number for PO box addresses.
struct postOfficeBoxNumber;

/// Only consensus opinion of experts, case studies, or standard-of-care.
struct EvidenceLevelC;

/// Boolean: True or False.
struct Boolean;

/// Specifies the Person that is legally accountable for the CreativeWork.
struct accountablePerson;

/// The muscle whose action counteracts the specified muscle.
struct antagonist;

/// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
struct brand;

/// This Review or Rating is relevant to this part or facet of the itemReviewed.
struct reviewAspect;

/// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is valid.\n\nSee also [[ineligibleRegion]].
///     
struct eligibleRegion;

/// A bakery.
struct Bakery;

/// The typical delay the order has been sent for delivery and the goods reach the final customer. Typical properties: minValue, maxValue, unitCode (d for DAY).
struct transitTime;

/// A musical composition.
struct MusicComposition;

/// The end time of the clip expressed as the number of seconds from the beginning of the work.
struct endOffset;

/// A specific branch of medical science that pertains to hereditary transmission and the variation of inherited characteristics and disorders.
struct Genetic;

/// Any FDA or other warnings about the drug (text or URL).
struct warning;

/// The drug's cost represents the retail cost of the drug.
struct Retail;

/// The lower value of some characteristic or property.
struct minValue;

/// A synagogue.
struct Synagogue;

/// Physical activity that is engaged in to improve muscle and bone strength. Also referred to as resistance training.
struct StrengthTraining;

/// The date on which a successful applicant for this job would be expected to start work. Choose a specific date in the future or use the jobImmediateStart property to indicate the position is to be filled as soon as possible.
struct jobStartDate;

/// A river (for example, the broad majestic Shannon).
struct RiverBodyOfWater;

/// The volume swept by all of the pistons inside the cylinders of an internal combustion engine in a single movement. \n\nTypical unit code(s): CMQ for cubic centimeter, LTR for liters, INQ for cubic inches\n* Note 1: You can link to information about how the given value has been determined using the [[valueReference]] property.\n* Note 2: You can use [[minValue]] and [[maxValue]] to indicate ranges.
struct engineDisplacement;

/// An offer to transfer some rights to an item or to provide a service — for example, an offer to sell tickets to an event, to rent the DVD of a movie, to stream a TV show over the internet, to repair a motorcycle, or to loan a book.\n\nNote: As the [[businessFunction]] property, which identifies the form of offer (e.g. sell, lease, repair, dispose), defaults to http://purl.org/goodrelations/v1#Sell; an Offer without a defined businessFunction value can be assumed to be an offer to sell.\n\nFor [GTIN](http://www.gs1.org/barcodes/technical/idkeys/gtin)-related fields, see [Check Digit calculator](http://www.gs1.org/barcodes/support/check_digit_calculator) and [validation guide](http://www.gs1us.org/resources/standards/gtin-validation-guide) from [GS1](http://www.gs1.org/).
struct Offer;

/// The most generic familial relation.
struct relatedTo;

/// RemixAlbum.
struct RemixAlbum;

/// The response (yes, no, maybe) to the RSVP.
struct rsvpResponse;

/// The total amount due.
struct totalPaymentDue;

/// A form of paying back money previously borrowed from a lender. Repayment usually takes the form of periodic payments that normally include part principal plus interest in each payment.
struct loanRepaymentForm;

/// An electronic file or document.
struct DigitalDocument;

/// Item(s) being shipped.
struct itemShipped;

/// TODO.
struct healthPlanDrugOption;

/// A specific branch of medical science that is concerned with the ear, nose and throat and their respective disease states.
struct Otolaryngologic;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserDownloads;

/// People or organizations that have reviewed the content on this web page for accuracy and/or completeness.
struct reviewedBy;

/// Statement about ethics policy, e.g. of a [[NewsMediaOrganization]] regarding journalistic and publishing practices, or of a [[Restaurant]], a page describing food source policies. In the case of a [[NewsMediaOrganization]], an ethicsPolicy is typically a statement describing the personal, organizational, and corporate standards of behavior expected by the organization.
struct ethicsPolicy;

/// An [[OfferForPurchase]] in Schema.org represents an [[Offer]] to sell something, i.e. an [[Offer]] whose
///   [[businessFunction]] is [sell](http://purl.org/goodrelations/v1#Sell.). See [Good Relations](https://en.wikipedia.org/wiki/GoodRelations) for
///   background on the underlying concepts.
///   
struct OfferForPurchase;

/// A CreativeWork attached to the message.
struct messageAttachment;

/// Used to describe a ticket to an event, a flight, a bus ride, etc.
struct Ticket;

/// Audiences defined by a person's maximum age.
struct requiredMaxAge;

/// An internet cafe.
struct InternetCafe;

/// Represents the collection of all sports organizations, including sports teams, governing bodies, and sports associations.
struct SportsOrganization;

/// A URL to a map of the place.
struct maps;

/// A preventative therapy used to prevent reoccurrence of the medical condition after an initial episode of the condition.
struct secondaryPrevention;

/// The Occupation for the JobPosting.
struct relevantOccupation;

/// Active, but not recruiting new participants.
struct ActiveNotRecruiting;

/// How the procedure is performed.
struct howPerformed;

/// The system of medicine that includes this MedicalEntity, for example 'evidence-based', 'homeopathic', 'chiropractic', etc.
struct medicineSystem;

/// The act of discovering/finding an object.
struct DiscoverAction;

/// Cash, Credit Card, Cryptocurrency, Local Exchange Tradings System, etc.
struct paymentAccepted;

/// Description of the absorption and elimination of drugs, including their concentration (pharmacokinetics, pK) and biological effects (pharmacodynamics, pD).
struct clincalPharmacology;

/// A monetary value above which (or equal to) the shipping rate becomes free. Intended to be used via an [[OfferShippingDetails]] with [[shippingSettingsLink]] matching this [[ShippingRateSettings]].
struct freeShippingThreshold;

/// A health profession of a person formally educated and trained in the care of the sick or infirm person.
struct Nursing;

/// Identifies that this [[Trip]] is a subTrip of another Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
struct partOfTrip;

/// Products owned by the organization or person.
struct owns;

/// The geographic coordinates of a place or event.
struct GeoCoordinates;

/// A relationship between an organization and a department of that organization, also described as an organization (allowing different urls, logos, opening hours). For example: a store with a pharmacy, or a bakery with a cafe.
struct department;

/// An airport.
struct Airport;

/// Educational background needed for the position or Occupation.
struct educationRequirements;

/// The amount of work expected of students taking the course, often provided as a figure per week or per month, and may be broken down by type. For example, "2 hours of lectures, 1 hour of lab work and 3 hours of independent study per week".
struct courseWorkload;

/// The region in which the locality is, and which is in the country. For example, California or another appropriate first-level [Administrative division](https://en.wikipedia.org/wiki/List_of_administrative_divisions_by_country) 
struct addressRegion;

/// The Value-added Tax ID of the organization or person.
struct vatID;

/// A mosque.
struct Mosque;

/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that crosses it: "a crosses b: they have some but not all interior points in common, and the dimension of the intersection is less than that of at least one of them". As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoCrosses;

/// Nonprofit501c25: Non-profit type referring to Real Property Title-Holding Corporations or Trusts with Multiple Parents.
struct Nonprofit501c25;

/// Someone working for this organization.
struct employee;

/// A polygon is the area enclosed by a point-to-point path for which the starting and ending points are the same. A polygon is expressed as a series of four or more space delimited points where the first and final points are identical.
struct polygon;

/// Represents a relationship between two geometries (or the places they represent), relating a containing geometry to a contained geometry. "a contains b iff no points of b lie in the exterior of a, and at least one point of the interior of b lies in the interior of a". As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoContains;

/// An EducationalAudience.
struct EducationalAudience;

/// The status of a creative work in terms of its stage in a lifecycle. Example terms include Incomplete, Draft, Published, Obsolete. Some organizations define a set of terms for the stages of their publication lifecycle.
struct creativeWorkStatus;

/// The type of boarding policy used by the airline (e.g. zone-based or group-based).
struct boardingPolicy;

/// The type of educational or occupational program. For example, classroom, internship, alternance, etc..
struct programType;

/// NonprofitType enumerates several kinds of official non-profit types of which a non-profit organization can be.
struct NonprofitType;

/// The station from which the train departs.
struct departureStation;

/// Drugs that affect the test's results.
struct affectedBy;

/// An elementary school.
struct ElementarySchool;

/// The typical working hours for this job (e.g. 1st shift, night shift, 8am-5pm).
struct workHours;

/// A theater group or company, for example, the Royal Shakespeare Company or Druid Theatre.
struct TheaterGroup;

/// The supported content type(s) for an EntryPoint response.
struct contentType;

/// A contact location for a person's place of work.
struct workLocation;

/// The type of tissue sample required for the test.
struct tissueSample;

/// Networks covered by this plan.
struct includesHealthPlanNetwork;

/// The basic containment relation between a place and one that contains it.
struct containedIn;

/// The most generic uni-directional social relation.
struct follows;

/// Date of birth.
struct birthDate;

/// numc19mechventpats - HOSPITALIZED and VENTILATED: Patients hospitalized in an NHSN inpatient care location who have suspected or confirmed COVID-19 and are on a mechanical ventilator.
struct cvdNumC19MechVentPats;

/// The duration of validity of a permit or similar thing.
struct validFor;

/// Any special commitments associated with this job posting. Valid entries include VeteranCommit, MilitarySpouseCommit, etc.
struct specialCommitments;

/// exif data for this object.
struct exifData;

/// A type of blood vessel that specifically carries blood away from the heart.
struct Artery;

/// An embedded video object.
struct video;

/// Nose function assessment with clinical examination.
struct Nose;

/// numbedsocc - HOSPITAL INPATIENT BED OCCUPANCY: Total number of staffed inpatient beds that are occupied.
struct cvdNumBedsOcc;

/// Whether or not a property is mutable.  Default is false. Specifying this for a property that also has a value makes it act similar to a "hidden" input in an HTML form.
struct readonlyValue;

/// A dentist.
struct Dentist;

/// An identifier for the method of payment used (e.g. the last 4 digits of the credit card).
struct paymentMethodId;

/// The jurisdiction from which the legislation originates.
struct legislationJurisdiction;

/// A nightclub or discotheque.
struct NightClub;

/// A person or organization attending the event.
struct attendee;

/// Permission to write or edit the document.
struct WritePermission;

/// A director of e.g. tv, radio, movie, video games etc. content. Directors can be associated with individual items or with a series, episode, clip.
struct directors;

/// A wholesale store.
struct WholesaleStore;

/// Represents a sale price (usually active for a limited period) of an offered product.
struct SalePrice;

/// A person or organization that supports a thing through a pledge, promise, or financial contribution. e.g. a sponsor of a Medical Study or a corporate sponsor of an event.
struct sponsor;

/// Comments, typically from users.
struct comment;

/// A utility class that serves as the umbrella for a number of 'intangible' things in the medical space.
struct MedicalIntangible;

/// The invitee will not attend.
struct RsvpResponseNo;

/// A courthouse.
struct Courthouse;

/// Used to describe a seat, such as a reserved seat in an event reservation.
struct Seat;

/// The act of responding to a question/message asked/sent by the object. Related to [[AskAction]]\n\nRelated actions:\n\n* [[AskAction]]: Appears generally as an origin of a ReplyAction.
struct ReplyAction;

/// One or more messages between organizations or people on a particular topic. Individual messages can be linked to the conversation with isPartOf or hasPart properties.
struct Conversation;

/// The "spatial" property can be used in cases when more specific properties
/// (e.g. [[locationCreated]], [[spatialCoverage]], [[contentLocation]]) are not known to be appropriate.
struct spatial;

/// Information about coping or life related to the topic.
struct LivingWithHealthAspect;

/// Imaging technique used.
struct imagingTechnique;

/// People working for this organization.
struct employees;

/// The larger organization that this organization is a [[subOrganization]] of, if any.
struct parentOrganization;

/// A process of progressive physical care and rehabilitation aimed at improving a health condition.
struct PhysicalTherapy;

/// A [[CampingPitch]] is an individual place for overnight stay in the outdoors, typically being part of a larger camping site, or [[Campground]].\n\n
/// In British English a campsite, or campground, is an area, usually divided into a number of pitches, where people can camp overnight using tents or camper vans or caravans; this British English use of the word is synonymous with the American English expression campground. In American English the term campsite generally means an area where an individual, family, group, or military unit can pitch a tent or park a camper; a campground may contain many campsites.
/// (Source: Wikipedia see [https://en.wikipedia.org/wiki/Campsite](https://en.wikipedia.org/wiki/Campsite)).\n\n
/// See also the dedicated [document on the use of schema.org for marking up hotels and other forms of accommodations](/docs/hotels.html).
struct CampingPitch;

/// A series of [[Event]]s. Included events can relate with the series using the [[superEvent]] property.
/// 
/// An EventSeries is a collection of events that share some unifying characteristic. For example, "The Olympic Games" is a series, which
/// is repeated regularly. The "2012 London Olympics" can be presented both as an [[Event]] in the series "Olympic Games", and as an
/// [[EventSeries]] that included a number of sporting competitions as Events.
/// 
/// The nature of the association between the events in an [[EventSeries]] can vary, but typical examples could
/// include a thematic event series (e.g. topical meetups or classes), or a series of regular events that share a location, attendee group and/or organizers.
/// 
/// EventSeries has been defined as a kind of Event to make it easy for publishers to use it in an Event context without
/// worrying about which kinds of series are really event-like enough to call an Event. In general an EventSeries
/// may seem more Event-like when the period of time is compact and when aspects such as location are fixed, but
/// it may also sometimes prove useful to describe a longer-term series as an Event.
///    
struct EventSeries;

/// One of a set of signs and symptoms that can be used to distinguish this diagnosis from others in the differential diagnosis.
struct distinguishingSign;

/// An observational study is a type of medical study that attempts to infer the possible effect of a treatment through observation of a cohort of subjects over a period of time. In an observational study, the assignment of subjects into treatment groups versus control groups is outside the control of the investigator. This is in contrast with controlled studies, such as the randomized controlled trials represented by MedicalTrial, where each subject is randomly assigned to a treatment group or a control group before the start of the treatment.
struct MedicalObservationalStudy;

/// The age of the business.
struct yearsInOperation;

/// An url template (RFC6570) that will be used to construct the target of the execution of the action.
struct urlTemplate;

/// The color of the product.
struct color;

/// Any matter of defined composition that has discrete existence, whose origin may be biological, mineral or chemical.
struct Substance;

/// A medical test typically performed given this condition.
struct typicalTest;

/// A defence establishment, such as an army or navy base.
struct DefenceEstablishment;

/// A Defined Term contained in this term set.
struct hasDefinedTerm;

/// An electronics store.
struct ElectronicsStore;

/// HealthAspectEnumeration enumerates several aspects of health content online, each of which might be described using [[hasHealthAspect]] and [[HealthTopicContent]].
struct HealthAspectEnumeration;

/// A transit map.
struct TransitMap;

/// A sub property of location. The sports event where this action occurred.
struct sportsEvent;

/// An online or virtual location for attending events. For example, one may attend an online seminar or educational event. While a virtual location may be used as the location of an event, virtual locations should not be confused with physical locations in the real world.
struct VirtualLocation;

/// A body of structured information describing some topic(s) of interest.
struct Dataset;

/// The location of the reserved seat (e.g., 27).
struct seatNumber;

/// The postings that are part of this blog.
struct blogPosts;

/// The location(s) applicants can apply from. This is usually used for telecommuting jobs where the applicant does not need to be in a physical office. Note: This should not be used for citizenship or work visa requirements.
struct applicantLocationRequirements;

/// NonprofitSBBI: Non-profit type referring to a Social Interest Promoting Institution (NL).
struct NonprofitSBBI;

/// A description of the qualification, award, certificate, diploma or other occupational credential awarded as a consequence of successful completion of this course or program.
struct occupationalCredentialAwarded;

/// The primary artist for a work
///     	in a medium other than pencils or digital line art--for example, if the
///     	primary artwork is done in watercolors or digital paints.
struct artist;

/// Any physical activity engaged in for job-related purposes. Examples may include waiting tables, maid service, carrying a mailbag, picking fruits or vegetables, construction work, etc.
struct OccupationalActivity;

/// A supply consumed when performing the instructions for how to achieve a result.
struct HowToSupply;

/// The number of grams of fat.
struct fatContent;

/// Any medical intervention designed to prevent, treat, and cure human diseases and medical conditions, including both curative and palliative therapies. Medical therapies are typically processes of care relying upon pharmacotherapy, behavioral therapy, supportive therapy (with fluid or nutrition for example), or detoxification (e.g. hemodialysis) aimed at improving or preventing a health condition.
struct MedicalTherapy;

/// The regions where the media is allowed. If not specified, then it's assumed to be allowed everywhere. Specify the countries in [ISO 3166 format](http://en.wikipedia.org/wiki/ISO_3166).
struct regionsAllowed;

/// The science or practice of testing visual acuity and prescribing corrective lenses.
struct Optometric;

/// A publication event e.g. catch-up TV or radio podcast, during which a program is available on-demand.
struct OnDemandEvent;

/// Specifies specific carrier(s) requirements for the application (e.g. an application may only work on a specific carrier network).
struct carrierRequirements;

/// A data catalog which contains this dataset (this property was previously 'catalog', preferred name is now 'includedInDataCatalog').
struct includedDataCatalog;

/// The frame size of the video.
struct videoFrameSize;

/// A link to the ListItem that follows the current one.
struct nextItem;

/// [[ArchiveOrganization]] that holds, keeps or maintains the [[ArchiveComponent]].
struct holdingArchive;

/// The number of grams of sugar.
struct sugarContent;

/// A description of the postoperative procedures, care, and/or followups for this device.
struct postOp;

/// Indicates an item or CreativeWork that is part of this item, or CreativeWork (in some sense).
struct hasPart;

/// The kind of aircraft (e.g., "Boeing 747").
struct aircraft;

/// The unique identifier for the train.
struct trainNumber;

/// Reference to an asset (e.g., Barcode, QR code image or PDF) usable for entrance.
struct ticketToken;

/// Entities that have a somewhat fixed, physical extension.
struct Place;

/// Indicates (via enumerated options) the return fees policy for a MerchantReturnPolicy
struct returnFees;

/// An Offer which must be accepted before the user can perform the Action. For example, the user may need to buy a movie before being able to watch it.
struct expectsAcceptanceOf;

/// A defined range of postal codes.
struct postalCodeRange;

/// A zoo.
struct Zoo;

/// A sub property of object. The collection target of the action.
struct targetCollection;

/// A specific branch of medical science that pertains to therapeutic or cosmetic repair or re-formation of missing, injured or malformed tissues or body parts by manual and instrumental means.
struct PlasticSurgery;

/// ReturnShippingFees ...
struct ReturnShippingFees;

/// Enumerated options related to a ContactPoint.
struct ContactPointOption;

/// The associated telephone number is toll free.
struct TollFree;

/// A sub property of participant. The winner of the action.
struct winner;

/// The date of adoption or signature of the legislation. This is the date at which the text is officially aknowledged to be a legislation, even though it might not even be published or in force.
struct legislationDate;

/// A medical device used for diagnostic purposes.
struct Diagnostic;

/// Party placing the order or paying the invoice.
struct customer;

/// A photograph.
struct Photograph;

/// OrderStatus representing that an order is being processed.
struct OrderProcessing;

/// Languages in which subtitles/captions are available, in [IETF BCP 47 standard format](http://tools.ietf.org/html/bcp47).
struct subtitleLanguage;

/// Accountancy business.\n\nAs a [[LocalBusiness]] it can be described as a [[provider]] of one or more [[Service]]\(s).
///       
struct AccountingService;

/// Indicates that the item has limited availability.
struct LimitedAvailability;

/// A diet conforming to Hindu dietary practices, in particular, beef-free.
struct HinduDiet;

/// A number associated with a role in an organization, for example, the number on an athlete's jersey.
struct numberedPosition;

/// Nonprofit501c4: Non-profit type referring to Civic Leagues, Social Welfare Organizations, and Local Associations of Employees.
struct Nonprofit501c4;

/// numtotbeds - ALL HOSPITAL BEDS: Total number of all Inpatient and outpatient beds, including all staffed,ICU, licensed, and overflow (surge) beds used for inpatients or outpatients.
struct cvdNumTotBeds;

/// Indicates the mobility of a provided service (e.g. 'static', 'dynamic').
struct providerMobility;

/// A system of medicine focused on the relationship between the body's structure, mainly the spine, and its functioning.
struct Chiropractic;

/// The unit of the dose, e.g. 'mg'.
struct doseUnit;

/// A possible serious complication and/or serious side effect of this therapy. Serious adverse outcomes include those that are life-threatening; result in death, disability, or permanent damage; require hospitalization or prolong existing hospitalization; cause congenital anomalies or birth defects; or jeopardize the patient and may require medical or surgical intervention to prevent one of the outcomes in this definition.
struct seriousAdverseOutcome;

/// The away team in a sports event.
struct awayTeam;

/// A permission related to the access to this document (e.g. permission to read or write an electronic document). For a public document, specify a grantee with an Audience with audienceType equal to "public".
struct hasDigitalDocumentPermission;

/// A dance group&#x2014;for example, the Alvin Ailey Dance Theater or Riverdance.
struct DanceGroup;

/// Any feature associated or not with a medical condition. In medicine a symptom is generally subjective while a sign is objective.
struct MedicalSignOrSymptom;

/// A predefined value for a product characteristic, e.g. the power cord plug type 'US' or the garment sizes 'S', 'M', 'L', and 'XL'.
struct QualitativeValue;

/// Other co-agents that participated in the action indirectly. e.g. John wrote a book with *Steve*.
struct participant;

/// Event type: Sales event.
struct SaleEvent;

/// Real-wheel drive is a transmission layout where the engine drives the rear wheels.
struct RearWheelDriveConfiguration;

/// DemoAlbum.
struct DemoAlbum;

/// A specific branch of medical science that pertains to diagnosis and treatment of disorders of skin.
struct Dermatology;

/// A permission for a particular person or group to access a particular file.
struct DigitalDocumentPermission;

/// Systems of medical practice.
struct MedicineSystem;

/// A slogan or motto associated with the item.
struct slogan;

/// A textual description of known damages, both repaired and unrepaired.
struct knownVehicleDamages;

/// The number of downvotes this question, answer or comment has received from the community.
struct downvoteCount;

/// Subcategory of the application, e.g. 'Arcade Game'.
struct applicationSubCategory;

/// A card payment method of a particular brand or name.  Used to mark up a particular payment method and/or the financial product/service that supplies the card account.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#AmericanExpress\n* http://purl.org/goodrelations/v1#DinersClub\n* http://purl.org/goodrelations/v1#Discover\n* http://purl.org/goodrelations/v1#JCB\n* http://purl.org/goodrelations/v1#MasterCard\n* http://purl.org/goodrelations/v1#VISA
///        
struct CreditCard;

/// The payment has been received and processed.
struct PaymentComplete;

/// The value of the quantitative value or property value node.\n\n* For [[QuantitativeValue]] and [[MonetaryAmount]], the recommended type for values is 'Number'.\n* For [[PropertyValue]], it can be 'Text;', 'Number', 'Boolean', or 'StructuredValue'.\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
struct value;

/// Nutrition information about the recipe or menu item.
struct nutrition;

/// Indicates another legislation taken into account in this consolidated legislation (which is usually the product of an editorial process that revises the legislation). This property should be used multiple times to refer to both the original version or the previous consolidated version, and to the legislations making the change.
struct legislationConsolidates;

/// Vital signs are measures of various physiological functions in order to assess the most basic body functions.
struct VitalSign;

/// An audio file.
struct AudioObject;

/// Date the content expires and is no longer useful or available. For example a [[VideoObject]] or [[NewsArticle]] whose availability or relevance is time-limited, or a [[ClaimReview]] fact check whose publisher wants to indicate that it may no longer be relevant (or helpful to highlight) after some date.
struct expires;

/// The condition, complication, etc. influenced by this factor.
struct increasesRiskOf;

/// For a given health insurance plan, the specification for costs and coverage of prescription drugs. 
struct HealthPlanFormulary;

/// The Dun & Bradstreet DUNS number for identifying an organization or business person.
struct duns;

/// Represents the activation fee part of the total price for an offered product, for example a cellphone contract
struct ActivationFee;

/// A museum.
struct Museum;

/// The product that this structured value is referring to.
struct typeOfGood;

/// The identifier of the transaction.
struct orderNumber;

/// The act of deliberately creating/producing/generating/building a result out of the agent.
struct CreateAction;

/// The size of the business in annual revenue.
struct yearlyRevenue;

/// Results are available.
struct ResultsAvailable;

/// Quantities such as distance, time, mass, weight, etc. Particular instances of say Mass are entities like '3 Kg' or '4 milligrams'.
struct Quantity;

/// This can be marked 'true' to indicate that some published [[DeliveryTimeSettings]] or [[ShippingRateSettings]] are intended to apply to all [[OfferShippingDetails]] published by the same merchant, when referenced by a [[shippingSettingsLink]] in those settings. It is not meaningful to use a 'true' value for this property alongside a transitTimeLabel (for [[DeliveryTimeSettings]]) or shippingLabel (for [[ShippingRateSettings]]), since this property is for use with unlabelled settings.
struct isUnlabelledFallback;

/// The status of a reservation on hold pending an update like credit card number or flight changes.
struct ReservationHold;

/// Magnetic resonance imaging.
struct MRI;

/// A type of blood vessel that specifically carries blood to the heart.
struct Vein;

/// Event type: Festival.
struct Festival;

/// A music recording (track), usually a single song.
struct MusicRecording;

/// Indicates whether API is managed or unmanaged.
struct programmingModel;

/// Formularies covered by this plan.
struct includesHealthPlanFormulary;

/// An answer offered to a question; perhaps correct, perhaps opinionated or wrong.
struct Answer;

/// A public swimming pool.
struct PublicSwimmingPool;

/// A contact location for a person's residence.
struct homeLocation;

/// Something in medical science that pertains to infectious diseases i.e caused by bacterial, viral, fungal or parasitic infections.
struct Infectious;

/// Indicates if this web page element is the main subject of the page.
struct mainContentOfPage;

/// An additional name for a Person, can be used for a middle name.
struct additionalName;

/// Version of the software instance.
struct softwareVersion;

/// The rating for the content.\n\nUsage guidelines:\n\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
struct ratingValue;

/// A car wash business.
struct AutoWash;

/// A collection of datasets.
struct DataCatalog;

/// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
struct petsAllowed;

/// A collection of music tracks.
struct MusicAlbum;

/// The number or other unique designator assigned to a Report by the publishing organization.
struct reportNumber;

/// A specific branch of medical science that is concerned with the study, treatment, and prevention of mental illness, using both medical and psychological therapies.
struct Psychiatric;

/// The URL for sending a payment.
struct paymentUrl;

/// Enumerated categories of medical drug costs.
struct DrugCostCategory;

/// The currency of the price, or a price component when attached to [[PriceSpecification]] and its subtypes.\n\nUse standard formats: [ISO 4217 currency format](http://en.wikipedia.org/wiki/ISO_4217) e.g. "USD"; [Ticker symbol](https://en.wikipedia.org/wiki/List_of_cryptocurrencies) for cryptocurrencies e.g. "BTC"; well known names for [Local Exchange Tradings Systems](https://en.wikipedia.org/wiki/Local_exchange_trading_system) (LETS) and other currency types e.g. "Ithaca HOUR".
struct priceCurrency;

/// The organization issuing the ticket or permit.
struct issuedBy;

/// Event type: Sports event.
struct SportsEvent;

/// A CreativeWork such as an image, video, or audio clip shared as part of this posting.
struct sharedContent;

/// Design models for medical trials. Enumerated type.
struct MedicalTrialDesign;

/// HealthCare: this is a benefit for health care.
struct HealthCare;

/// The expected length of time to complete the program if attending full-time.
struct timeToComplete;

/// An airline flight.
struct Flight;

/// Source of the data used to formulate the guidance, e.g. RCT, consensus opinion, etc.
struct evidenceOrigin;

/// Game server status: OfflinePermanently. Server is offline and not available.
struct OfflinePermanently;

/// The work that this work has been translated from. e.g. 物种起源 is a translationOf “On the Origin of Species”
struct translationOfWork;

/// A sub property of participant. The person that lends the object being borrowed.
struct lender;

/// A medical guideline related to this entity.
struct guideline;

/// A list of possible statuses for the legal force of a legislation.
struct LegalForceStatus;

/// Any bodily activity that enhances or maintains physical fitness and overall health and wellness. Includes activity that is part of daily living and routine, structured exercise, and exercise prescribed as part of a medical treatment or recovery plan.
struct PhysicalActivity;

/// The invitee may or may not attend.
struct RsvpResponseMaybe;

/// A web page that provides medical information.
struct MedicalWebPage;

/// A [[NewsArticle]] providing historical context, definition and detail on a specific topic (aka "explainer" or "backgrounder"). For example, an in-depth article or frequently-asked-questions ([FAQ](https://en.wikipedia.org/wiki/FAQ)) document on topics such as Climate Change or the European Union. Other kinds of background material from a non-news setting are often described using [[Book]] or [[Article]], in particular [[ScholarlyArticle]]. See also [[NewsArticle]] for related vocabulary from a learning/education perspective.
struct BackgroundNewsArticle;

/// Shipper's address.
struct originAddress;

/// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
struct validThrough;

/// The title of the job.
struct title;

/// The predominant mode of learning supported by the learning resource. Acceptable values are 'active', 'expositive', or 'mixed'.
struct interactivityType;

/// An alignment to an established educational framework.
/// 
/// This property should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource [[teaches]] or [[assesses]] a competency.
struct educationalAlignment;

/// A BreadcrumbList is an ItemList consisting of a chain of linked Web pages, typically described using at least their URL and their name, and typically ending with the current page.\n\nThe [[position]] property is used to reconstruct the order of the items in a BreadcrumbList The convention is that a breadcrumb list has an [[itemListOrder]] of [[ItemListOrderAscending]] (lower values listed first), and that the first items in this list correspond to the "top" or beginning of the breadcrumb trail, e.g. with a site or section homepage. The specific values of 'position' are not assigned meaning for a BreadcrumbList, but they should be integers, e.g. beginning with '1' for the first item in the list.
///       
struct BreadcrumbList;

/// An event happening at a certain time and location, such as a concert, lecture, or festival. Ticketing information may be added via the [[offers]] property. Repeated events may be structured as separate Event objects.
struct Event;

/// An indication for treating an underlying condition, symptom, etc.
struct TreatmentIndication;

/// The variableMeasured property can indicate (repeated as necessary) the  variables that are measured in some dataset, either described as text or as pairs of identifier and description using PropertyValue.
struct variableMeasured;

/// A set of defined terms for example a set of categories or a classification scheme, a glossary, dictionary or enumeration.
struct DefinedTermSet;

/// The illustrator of the book.
struct illustrator;

/// The act of playing/exercising/training/performing for enjoyment, leisure, recreation, Competition or exercise.\n\nRelated actions:\n\n* [[ListenAction]]: Unlike ListenAction (which is under ConsumeAction), PlayAction refers to performing for an audience or at an event, rather than consuming music.\n* [[WatchAction]]: Unlike WatchAction (which is under ConsumeAction), PlayAction refers to showing/displaying for an audience or at an event, rather than consuming visual content.
struct PlayAction;

/// The page on which the work ends; for example "138" or "xvi".
struct pageEnd;

/// Any precaution, guidance, contraindication, etc. related to this drug's use by breastfeeding mothers.
struct breastfeedingWarning;

/// A DeliveryTimeSettings represents re-usable pieces of shipping information, relating to timing. It is designed for publication on an URL that may be referenced via the [[shippingSettingsLink]] property of a [[OfferShippingDetails]]. Several occurrences can be published, distinguished (and identified/referenced) by their different values for [[transitTimeLabel]].
struct DeliveryTimeSettings;

/// A sub property of object. The options subject to this action.
struct option;

/// A list of single or combined accessModes that are sufficient to understand all the intellectual content of a resource. Expected values include:  auditory, tactile, textual, visual.
///       
struct accessModeSufficient;

/// An [[OpinionNewsArticle]] is a [[NewsArticle]] that primarily expresses opinions rather than journalistic reporting of news and events. For example, a [[NewsArticle]] consisting of a column or [[Blog]]/[[BlogPosting]] entry in the Opinions section of a news publication. 
struct OpinionNewsArticle;

/// A set of requirements that a must be fulfilled in order to perform an Action.
struct ActionAccessSpecification;

/// An [EIDR](https://eidr.org/) (Entertainment Identifier Registry) [[identifier]] representing at the most general/abstract level, a work of film or television.
/// 
/// For example, the motion picture known as "Ghostbusters" has a titleEIDR of  "10.5240/7EC7-228A-510A-053E-CBB8-J". This title (or work) may have several variants, which EIDR calls "edits". See [[editEIDR]].
/// 
/// Since schema.org types like [[Movie]] and [[TVEpisode]] can be used for both works and their multiple expressions, it is possible to use [[titleEIDR]] alone (for a general description), or alongside [[editEIDR]] for a more edit-specific description.
struct titleEIDR;

/// A US-style health insurance plan network. 
struct HealthPlanNetwork;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserCheckins;

/// Specifying something physically contained by something else. Typically used here for the underlying anatomical structures, such as organs, that comprise the anatomical system.
struct comprisedOf;

/// A piece of data that represents a particular aspect of a fictional character (skill, power, character points, advantage, disadvantage).
struct characterAttribute;

/// Operating systems supported (Windows 7, OSX 10.6, Android 1.6).
struct operatingSystem;

/// A HyperToEntry is an item within a [[HyperToc]], which represents a hypertext table of contents for complex media objects, such as [[VideoObject]], [[AudioObject]]. The media object itself is indicated using [[associatedMedia]]. Each section of interest within that content can be described with a [[HyperTocEntry]], with associated [[startOffset]] and [[endOffset]]. When several entries are all from the same file, [[associatedMedia]] is used on the overarching [[HyperTocEntry]]; if the content has been split into multiple files, they can be referenced using [[associatedMedia]] on each [[HyperTocEntry]].
struct HyperTocEntry;

/// The artwork on the cover of a comic.
struct ComicCoverArt;

/// The capacity of the fuel tank or in the case of electric cars, the battery. If there are multiple components for storage, this should indicate the total of all storage of the same type.\n\nTypical unit code(s): LTR for liters, GLL of US gallons, GLI for UK / imperial gallons, AMH for ampere-hours (for electrical vehicles).
struct fuelCapacity;

/// The date after which the price is no longer available.
struct priceValidUntil;

/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that geospatially overlaps it, i.e. they have some but not all points in common. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoOverlaps;

/// The URL of a node in an established educational framework.
struct targetUrl;

/// Specifics about the observational study design (enumerated).
struct studyDesign;

/// A play is a form of literature, usually consisting of dialogue between characters, intended for theatrical performance rather than just reading. Note the peformance of a Play would be a [[TheaterEvent]] - the *Play* being the [[workPerformed]].
struct Play;

/// An agent quotes/estimates/appraises an object/product/service with a price at a location/store.
struct QuoteAction;

/// Indicates that the item is available for pre-order.
struct PreOrder;

/// A tattoo parlor.
struct TattooParlor;

/// The act of posing a question / favor to someone.\n\nRelated actions:\n\n* [[ReplyAction]]: Appears generally as a response to AskAction.
struct AskAction;

/// A comedy club.
struct ComedyClub;

/// The edition of the print product in which the NewsArticle appears.
struct printEdition;

/// A process of care using radiation aimed at improving a health condition.
struct RadiationTherapy;

/// The person or organization that originally passed or made the law : typically parliament (for primary legislation) or government (for secondary legislation). This indicates the "legal author" of the law, as opposed to its physical author.
struct legislationPassedBy;

/// The special opening hours of a certain place.\n\nUse this to explicitly override general opening hours brought in scope by [[openingHoursSpecification]] or [[openingHours]].
///       
struct specialOpeningHoursSpecification;

/// A physical examination that can identify this sign.
struct identifyingExam;

/// The act of notifying an event organizer as to whether you expect to attend the event.
struct RsvpAction;

/// A simple system that adds up the number of risk factors to yield a score that is associated with prognosis, e.g. CHAD score, TIMI risk score.
struct MedicalRiskScore;

/// An advertising section of the page.
struct WPAdBlock;

/// The generic name of this drug or supplement.
struct nonProprietaryName;

/// An embedded audio object.
struct audio;

/// The composition this track is a recording of.
struct recordingOf;

/// Genre of the creative work, broadcast channel or group.
struct genre;

/// Whether borrower is a resident of the jurisdiction where the property is located.
struct domiciledMortgage;

/// A mathematical expression (e.g. 'x^2-3x=0') that may be solved for a specific variable, simplified, or transformed. This can take many formats, e.g. LaTeX, Ascii-Math, or math as you would write with a keyboard.
struct mathExpression;

/// A 3D model represents some kind of 3D content, which may have [[encoding]]s in one or more [[MediaObject]]s. Many 3D formats are available (e.g. see [Wikipedia](https://en.wikipedia.org/wiki/Category:3D_graphics_file_formats)); specific encoding formats can be represented using the [[encodingFormat]] property applied to the relevant [[MediaObject]]. For the
/// case of a single file published after Zip compression, the convention of appending '+zip' to the [[encodingFormat]] can be used. Geospatial, AR/VR, artistic/animation, gaming, engineering and scientific content can all be represented using [[3DModel]].
struct ThreeDModel;

/// A company or fund that gathers capital from a number of investors to create a pool of money that is then re-invested into stocks, bonds and other assets.
struct InvestmentFund;

/// Permission to read or view the document.
struct ReadPermission;

/// The measuredProperty of an [[Observation]], either a schema.org property, a property from other RDF-compatible systems e.g. W3C RDF Data Cube, or schema.org extensions such as [GS1's](https://www.gs1.org/voc/?show=properties).
struct measuredProperty;

/// Link to a page containing [[ShippingRateSettings]] and [[DeliveryTimeSettings]] details.
struct shippingSettingsLink;

/// The GTIN-12 code of the product, or the product to which the offer refers. The GTIN-12 is the 12-digit GS1 Identification Key composed of a U.P.C. Company Prefix, Item Reference, and Check Digit used to identify trade items. See [GS1 GTIN Summary](http://www.gs1.org/barcodes/technical/idkeys/gtin) for more details.
struct gtin12;

/// A specific branch of medical science that pertains to the study of the kidneys and its respective disease states.
struct Renal;

/// The individual who traces over the pencil drawings in ink after pencils are complete.
struct inker;

/// Number of players on the server.
struct playersOnline;

/// The answer(s) that has been accepted as best, typically on a Question/Answer site. Sites vary in their selection mechanisms, e.g. drawing on community opinion and/or the view of the Question author.
struct acceptedAnswer;

/// The basic data types such as Integers, Strings, etc.
struct DataType;

/// A secondary contributor to the CreativeWork or Event.
struct contributor;

/// A toy store.
struct ToyStore;

/// Indicates that this legislation (or part of a legislation) somehow transfers another legislation in a different legislative context. This is an informative link, and it has no legal value. For legally-binding links of transposition, use the <a href="/legislationTransposes">legislationTransposes</a> property. For example an informative consolidated law of a European Union's member state "applies" the consolidated version of the European Directive implemented in it.
struct legislationApplies;

/// A posting that is part of this blog.
struct blogPost;

/// A value indicating a special usage of a car, e.g. commercial rental, driving school, or as a taxi.
struct CarUsageType;

/// A designation that the drug in question has not been assigned a pregnancy category designation by the US FDA.
struct FDAnotEvaluated;

/// This property links to all [[UnitPriceSpecification]] nodes that apply in parallel for the [[CompoundPriceSpecification]] node.
struct priceComponent;

/// Four-wheel drive is a transmission layout where the engine primarily drives two wheels with a part-time four-wheel drive capability.
struct FourWheelDriveConfiguration;

/// A large, usually printed placard, bill, or announcement, often illustrated, that is posted to advertise or publicize something.
struct Poster;

/// A city or town.
struct City;

/// The minimum payment required at this time.
struct minimumPaymentDue;

/// The delivery method(s) to which the delivery charge or payment charge specification applies.
struct appliesToDeliveryMethod;

/// The tier(s) of drugs offered by this formulary or insurance plan.
struct healthPlanDrugTier;

/// Was the offer accepted as a gift for someone other than the buyer.
struct isGift;

/// The act of expressing a consistency of opinion with the object. An agent agrees to/about an object (a proposition, topic or theme) with participants.
struct AgreeAction;

/// Indicates if use of the media require a subscription  (either paid or free). Allowed values are ```true``` or ```false``` (note that an earlier version had 'yes', 'no').
struct requiresSubscription;

/// Indicates the party responsible for generating and publishing the current structured data markup, typically in cases where the structured data is derived automatically from existing published content but published on a different site. For example, student projects and open data initiatives often re-publish existing content with more explicitly structured metadata. The
/// [[sdPublisher]] property helps make such practices more explicit.
struct sdPublisher;

/// A media season e.g. tv, radio, video game etc.
struct Season;

/// Size of the application / package (e.g. 18MB). In the absence of a unit (MB, KB etc.), KB will be assumed.
struct fileSize;

/// A GeoCircle is a GeoShape representing a circular geographic area. As it is a GeoShape
///           it provides the simple textual property 'circle', but also allows the combination of postalCode alongside geoRadius.
///           The center of the circle can be indicated via the 'geoMidpoint' property, or more approximately using 'address', 'postalCode'.
///        
struct GeoCircle;

/// Any information related to overdose on a drug, including signs or symptoms, treatments, contact information for emergency response.
struct overdosage;

/// An intended audience, i.e. a group for whom something was created.
struct audience;

/// Indicates the usage of the vehicle for driving school.
struct DrivingSchoolVehicleUsage;

/// The date the ticket was issued.
struct dateIssued;

/// An agent controls a device or application.
struct ControlAction;

/// An entity represented by an entry in a list or data feed (e.g. an 'artist' in a list of 'artists')’.
struct item;

/// The likely outcome in either the short term or long term of the medical condition.
struct expectedPrognosis;

/// The number of persons that can be seated (e.g. in a vehicle), both in terms of the physical space available, and in terms of limitations set by law.\n\nTypical unit code(s): C62 for persons 
struct seatingCapacity;

/// The steering position is on the right side of the vehicle (viewed from the main direction of driving).
struct RightHandDriving;

/// A garden store.
struct GardenStore;

/// LimitedByGuaranteeCharity: Non-profit type referring to a charitable company that is limited by guarantee (UK).
struct LimitedByGuaranteeCharity;

/// The overall rating, based on a collection of reviews or ratings, of the item.
struct aggregateRating;

/// Event type: Literary event.
struct LiteraryEvent;

/// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication.
struct userInteractionCount;

/// A thesis or dissertation document submitted in support of candidature for an academic degree or professional qualification.
struct Thesis;

/// A diet exclusive of animal meat.
struct VegetarianDiet;

/// A medical science pertaining to chemical, hematological, immunologic, microscopic, or bacteriological diagnostic analyses or research
struct LaboratoryScience;

/// A [[CriticReview]] is a more specialized form of Review written or published by a source that is recognized for its reviewing activities. These can include online columns, travel and food guides, TV and radio shows, blogs and other independent Web sites. [[CriticReview]]s are typically more in-depth and professionally written. For simpler, casually written user/visitor/viewer/customer reviews, it is more appropriate to use the [[UserReview]] type. Review aggregator sites such as Metacritic already separate out the site's user reviews from selected critic reviews that originate from third-party sources.
struct CriticReview;

/// A house painting service.
struct HousePainter;

/// A media object representing the circumstances after performing this direction.
struct afterMedia;

/// Password, PIN, or access code needed for delivery (e.g. from a locker).
struct accessCode;

/// An XPath, e.g. of a [[SpeakableSpecification]] or [[WebPageElement]]. In the latter case, multiple matches within a page can constitute a single conceptual "Web page element".
struct xpath;

/// A roofing contractor.
struct RoofingContractor;

/// A sub property of object. The collection target of the action.
struct collection;

/// A short band of tough, flexible, fibrous connective tissue that functions to connect multiple bones, cartilages, and structurally support joints.
struct Ligament;

/// A sidebar section of the page.
struct WPSideBar;

/// Indicates whether this content is family friendly.
struct isFamilyFriendly;

/// Indicates a target EntryPoint for an Action.
struct target;

/// Whether The copay amount.
struct healthPlanCopay;

/// An application that can complete the request.
struct actionApplication;

/// A utility class that serves as the umbrella for a number of 'intangible' things such as quantities, structured values, etc.
struct Intangible;

/// Represents spatial relations in which two geometries (or the places they represent) are topologically equal, as defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM). "Two geometries are topologically equal if their interiors intersect and no part of the interior or boundary of one geometry intersects the exterior of the other" (a symmetric relationship)
struct geoEquals;

/// The object upon which the action is carried out, whose state is kept intact or changed. Also known as the semantic roles patient, affected or undergoer (which change their state) or theme (which doesn't). e.g. John read *a book*.
struct object;

/// Description of skills and experience needed for the position or Occupation.
struct experienceRequirements;

/// EnergyConsumptionDetails represents information related to the energy efficiency of a product that consumes energy. The information that can be provided is based on international regulations such as for example [EU directive 2017/1369](https://eur-lex.europa.eu/eli/reg/2017/1369/oj) for energy labeling and the [Energy labeling rule](https://www.ftc.gov/enforcement/rules/rulemaking-regulatory-reform-proceedings/energy-water-use-labeling-consumer) under the Energy Policy and Conservation Act (EPCA) in the US
struct EnergyConsumptionDetails;

/// A QAPage is a WebPage focussed on a specific Question and its Answer(s), e.g. in a question answering site or documenting Frequently Asked Questions (FAQs).
struct QAPage;

/// EventStatusType is an enumeration type whose instances represent several states that an Event may be in.
struct EventStatusType;

/// True is the broadcast is of a live event.
struct isLiveBroadcast;

/// A delivery method is a standardized procedure for transferring the product or service to the destination of fulfillment chosen by the customer. Delivery methods are characterized by the means of transportation used, and by the organization or group that is the contracting party for the sending organization or person.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#DeliveryModeDirectDownload\n* http://purl.org/goodrelations/v1#DeliveryModeFreight\n* http://purl.org/goodrelations/v1#DeliveryModeMail\n* http://purl.org/goodrelations/v1#DeliveryModeOwnFleet\n* http://purl.org/goodrelations/v1#DeliveryModePickUp\n* http://purl.org/goodrelations/v1#DHL\n* http://purl.org/goodrelations/v1#FederalExpress\n* http://purl.org/goodrelations/v1#UPS
///         
struct DeliveryMethod;

/// An office equipment store.
struct OfficeEquipmentStore;

/// The act of participating in performance arts.
struct PerformAction;

/// Description of fees, commissions, and other terms applied either to a class of financial product, or by a financial service organization.
struct feesAndCommissionsSpecification;

/// An honorific suffix following a Person's name such as M.D. /PhD/MSCSW.
struct honorificSuffix;

/// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
struct isicV4;

/// For questions that are part of learning resources (e.g. Quiz), eduQuestionType indicates the format of question being given. Example: "Multiple choice", "Open ended", "Flashcard".
struct eduQuestionType;

/// A permit issued by an organization, e.g. a parking pass.
struct Permit;

/// A movie.
struct Movie;

/// A system of medicine focused on promoting the body's innate ability to heal itself.
struct Osteopathic;

/// The parent of a question, answer or item in general.
struct parentItem;

/// A middle school (typically for children aged around 11-14, although this varies somewhat).
struct MiddleSchool;

/// A Global Trade Item Number ([GTIN](https://www.gs1.org/standards/id-keys/gtin)). GTINs identify trade items, including products and services, using numeric identification codes. The [[gtin]] property generalizes the earlier [[gtin8]], [[gtin12]], [[gtin13]], and [[gtin14]] properties. The GS1 [digital link specifications](https://www.gs1.org/standards/Digital-Link/) express GTINs as URLs. A correct [[gtin]] value should be a valid GTIN, which means that it should be an all-numeric string of either 8, 12, 13 or 14 digits, or a "GS1 Digital Link" URL based on such a string. The numeric component should also have a [valid GS1 check digit](https://www.gs1.org/services/check-digit-calculator) and meet the other rules for valid GTINs. See also [GS1's GTIN Summary](http://www.gs1.org/barcodes/technical/idkeys/gtin) and [Wikipedia](https://en.wikipedia.org/wiki/Global_Trade_Item_Number) for more details. Left-padding of the gtin values is not required or encouraged.
///    
struct gtin;

/// A Workers Union (also known as a Labor Union, Labour Union, or Trade Union) is an organization that promotes the interests of its worker members by collectively bargaining with management, organizing, and political lobbying.
struct WorkersUnion;

/// An audiobook.
struct Audiobook;

/// The act of forming a personal connection with someone (object) mutually/bidirectionally/symmetrically.\n\nRelated actions:\n\n* [[FollowAction]]: Unlike FollowAction, BefriendAction implies that the connection is reciprocal.
struct BefriendAction;

/// A MerchantReturnPolicy provides information about product return policies associated with an [[Organization]] or [[Product]].
struct MerchantReturnPolicy;

/// A fast-food restaurant.
struct FastFoodRestaurant;

/// A food-related business.
struct FoodEstablishment;

/// A review of an item - for example, of a restaurant, movie, or store.
struct Review;

/// For a [[NewsMediaOrganization]], a statement on coverage priorities, including any public agenda or stance on issues.
struct missionCoveragePrioritiesPolicy;

/// The section location of the reserved seat (e.g. Orchestra).
struct seatSection;

/// A financial product for the loaning of an amount of money, or line of credit, under agreed terms and charges.
struct LoanOrCredit;

/// A profession, may involve prolonged training and/or a formal qualification.
struct Occupation;

/// The terminal or port from which the boat departs.
struct departureBoatTerminal;

/// The cost per unit of the drug.
struct costPerUnit;

/// a type of payment made in cash during the onset of the purchase of an expensive good/service. The payment typically represents only a percentage of the full purchase price.
struct downPayment;

/// Indicates that the item is available only online.
struct OnlineOnly;

/// Any anatomical structure which pertains to the soft nervous tissue functioning as the coordinating center of sensation and intellectual and nervous activity.
struct BrainStructure;

/// The event has been cancelled. If the event has multiple startDate values, all are assumed to be cancelled. Either startDate or previousStartDate may be used to specify the event's cancelled date(s).
struct EventCancelled;

/// The number of copies when multiple copies of a piece of artwork are produced - e.g. for a limited edition of 20 prints, 'artEdition' refers to the total number of copies (in this example "20").
struct artEdition;

/// The item being described is intended to help a person learn the competency or learning outcome defined by the referenced term.
struct teaches;

/// The official name of the organization, e.g. the registered company name.
struct legalName;

/// A software application designed specifically to work well on a mobile device such as a telephone.
struct MobileApplication;

/// One of a set of differential diagnoses for the condition. Specifically, a closely-related or competing diagnosis typically considered later in the cognitive process whereby this medical condition is distinguished from others most likely responsible for a similar collection of signs and symptoms to reach the most parsimonious diagnosis or diagnoses in a patient.
struct differentialDiagnosis;

/// Neck assessment with clinical examination.
struct Neck;

/// A part of a successively published publication such as a periodical or publication volume, often numbered, usually containing a grouping of works such as articles.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
struct PublicationIssue;

/// The "temporal" property can be used in cases where more specific properties
/// (e.g. [[temporalCoverage]], [[dateCreated]], [[dateModified]], [[datePublished]]) are not known to be appropriate.
struct temporal;

/// Intended audience for an item, i.e. the group for whom the item was created.
struct Audience;

/// A collection or bound volume of maps, charts, plates or tables, physical or in media form illustrating any subject.
struct Atlas;

/// Any offered product or service. For example: a pair of shoes; a concert ticket; the rental of a car; a haircut; or an episode of a TV show streamed online.
struct Product;

/// A means of accessing the service (e.g. a phone bank, a web site, a location, etc.).
struct availableChannel;

/// A moving company.
struct MovingCompany;

/// A government building.
struct GovernmentBuilding;

/// The maximum physical attendee capacity of an [[Event]] whose [[eventAttendanceMode]] is [[OfflineEventAttendanceMode]] (or the offline aspects, in the case of a [[MixedEventAttendanceMode]]). 
struct maximumPhysicalAttendeeCapacity;

/// The geographic shape of a place. A GeoShape can be described using several properties whose values are based on latitude/longitude pairs. Either whitespace or commas can be used to separate latitude and longitude; whitespace should be used when writing a list of several such points.
struct GeoShape;

/// A specific object or file containing a Legislation. Note that the same Legislation can be published in multiple files. For example, a digitally signed PDF, a plain PDF and an HTML version.
struct LegislationObject;

/// Enumerated for values for itemListOrder for indicating how an ordered ItemList is organized.
struct ItemListOrderType;

/// The practice or art and science of preparing and dispensing drugs and medicines.
struct PharmacySpecialty;

/// A diet exclusive of all animal products.
struct VeganDiet;

/// Nonprofit501c12: Non-profit type referring to Benevolent Life Insurance Associations, Mutual Ditch or Irrigation Companies, Mutual or Cooperative Telephone Companies.
struct Nonprofit501c12;

/// The season to which this episode belongs.
struct partOfSeason;

/// A short summary of the specific claims reviewed in a ClaimReview.
struct claimReviewed;

/// A single item within a larger data feed.
struct DataFeedItem;

/// Neurological system clinical examination.
struct Neuro;

/// A characteristic of the described resource that is physiologically dangerous to some users. Related to WCAG 2.0 guideline 2.3 ([WebSchemas wiki lists possible values](http://www.w3.org/wiki/WebSchemas/Accessibility)).
struct accessibilityHazard;

/// Any complaint sensed and expressed by the patient (therefore defined as subjective)  like stomachache, lower-back pain, or fatigue.
struct MedicalSymptom;

/// An agent bookmarks/flags/labels/tags/marks an object.
struct BookmarkAction;

/// Web page type: Checkout page.
struct CheckoutPage;

/// The supporting materials for the artwork, e.g. Canvas, Paper, Wood, Board, etc.
struct artworkSurface;

/// Indicates that the resource is compatible with the referenced accessibility API ([WebSchemas wiki lists possible values](http://www.w3.org/wiki/WebSchemas/Accessibility)).
struct accessibilityAPI;

/// Indications regarding the permitted usage of the accommodation.
struct permittedUsage;

/// The footer section of the page.
struct WPFooter;

/// A service to convert funds from one currency to another currency.
struct CurrencyConversionService;

/// The neurological pathway that originates the neurons.
struct sourcedFrom;

/// Represents a relationship between two geometries (or the places they represent), relating a geometry to one that contains it, i.e. it is inside (i.e. within) its interior. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoWithin;

/// A radio episode which can be part of a series or season.
struct RadioEpisode;

/// The date and time the reservation was booked.
struct bookingTime;

/// A line is a point-to-point path consisting of two or more points. A line is expressed as a series of two or more point objects separated by space.
struct line;

/// Indicates a page documenting how licenses can be purchased or otherwise acquired, for the current item.
struct acquireLicensePage;

/// The total delay between the receipt of the order and the goods reaching the final customer.
struct deliveryTime;

/// Lists or enumerations—for example, a list of cuisines or music genres, etc.
struct Enumeration;

/// The expected salary upon completing the training.
struct salaryUponCompletion;

/// A unique identifier for the membership.
struct membershipNumber;

/// Nonprofit501c6: Non-profit type referring to Business Leagues, Chambers of Commerce, Real Estate Boards.
struct Nonprofit501c6;

/// A person that acts as performing member of a sports team; a player as opposed to a coach.
struct athlete;

/// An electrician.
struct Electrician;

/// Nutritional information about the recipe.
struct NutritionInformation;

/// A Role that represents a Web link e.g. as expressed via the 'url' property. Its linkRelationship property can indicate URL-based and plain textual link types e.g. those in IANA link registry or others such as 'amphtml'. This structure provides a placeholder where details from HTML's link element can be represented outside of HTML, e.g. in JSON-LD feeds.
struct LinkRole;

/// A specific dosing schedule for a drug or supplement.
struct DoseSchedule;

/// Event type: Theater performance.
struct TheaterEvent;

/// The event being broadcast such as a sporting event or awards ceremony.
struct broadcastOfEvent;

/// Nonprofit501c21: Non-profit type referring to Black Lung Benefit Trusts.
struct Nonprofit501c21;

/// An agent orders a (not yet released) object/product/service to be delivered/sent.
struct PreOrderAction;

/// Treatments or related therapies for a Topic.
struct TreatmentsHealthAspect;

/// How often the dose is taken, e.g. 'daily'.
struct frequency;

/// Indicates the total (available plus unavailable) number of accommodation units in an [[ApartmentComplex]], or the number of accommodation units for a specific [[FloorPlan]] (within its specific [[ApartmentComplex]]). See also [[numberOfAvailableAccommodationUnits]].
struct numberOfAccommodationUnits;

/// A specific branch of medical science that pertains to diagnosis and treatment of disorders of heart and vasculature.
struct Cardiovascular;

/// Category of an [[Accommodation]], following real estate conventions e.g. RESO (see [PropertySubType](https://ddwiki.reso.org/display/DDW17/PropertySubType+Field), and [PropertyType](https://ddwiki.reso.org/display/DDW17/PropertyType+Field) fields  for suggested values).
struct accommodationCategory;

/// A ExchangeRefund ...
struct ExchangeRefund;

/// A reservation for an event like a concert, sporting event, or lecture.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
struct EventReservation;

/// A sub property of instrument. The query used on this action.
struct query;

/// OrderStatus representing cancellation of an order.
struct OrderCancelled;

/// A sub property of object. The person or organization being followed.
struct followee;

/// A payment method using a credit, debit, store or other card to associate the payment with an account.
struct PaymentCard;

/// A statement of knowledge, skill, ability, task or any other assertion expressing a competency that is desired or required to fulfill this role or to work in this occupation.
struct skills;

/// A monetary value or range. This type can be used to describe an amount of money such as $50 USD, or a range as in describing a bank account being suitable for a balance between £1,000 and £1,000,000 GBP, or the value of a salary, etc. It is recommended to use [[PriceSpecification]] Types to describe the price of an Offer, Invoice, etc.
struct MonetaryAmount;

/// An educational organization.
struct EducationalOrganization;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserInteraction;

/// A specific branch of medical science that is concerned with the diagnosis and treatment of diseases pertaining to the urinary tract and the urogenital system.
struct Urologic;

/// CreativeWorkSeries dedicated to radio broadcast and associated online delivery.
struct RadioSeries;

/// An [[EmployerReview]] is a review of an [[Organization]] regarding its role as an employer, written by a current or former employee of that organization.
struct EmployerReview;

/// numbeds - HOSPITAL INPATIENT BEDS: Inpatient beds, including all staffed, licensed, and overflow (surge) beds used for inpatients.
struct cvdNumBeds;

/// An official rating for a lodging business or food establishment, e.g. from national associations or standards bodies. Use the author property to indicate the rating organization, e.g. as an Organization with name such as (e.g. HOTREC, DEHOGA, WHR, or Hotelstars).
struct starRating;

/// Indicates the GeoCoordinates at the centre of a GeoShape e.g. GeoCircle.
struct geoMidpoint;

/// The act of conveying information to another person via a communication medium (instrument) such as speech, email, or telephone conversation.
struct CommunicateAction;

/// A diet appropriate for people with diabetes.
struct DiabeticDiet;

/// Order cutoff time allows merchants to describe the time after which they will no longer process orders received on that day. For orders processed after cutoff time, one day gets added to the delivery time estimate. This property is expected to be most typically used via the [[ShippingRateSettings]] publication pattern. The time is indicated using the ISO-8601 Time format, e.g. "23:30:00-05:00" would represent 6:30 pm Eastern Standard Time (EST) which is 5 hours behind Coordinated Universal Time (UTC).
struct cutoffTime;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserComments;

/// Component dependency requirements for application. This includes runtime environments and shared libraries that are not included in the application distribution package, but required to run the application (Examples: DirectX, Java or .NET runtime).
struct softwareRequirements;

/// The act of giving money to a seller in exchange for goods or services rendered. An agent buys an object, product, or service from a seller for a price. Reciprocal of SellAction.
struct BuyAction;

/// Defines the month(s) of the year on which a recurring [[Event]] takes place. Specified as an [[Integer]] between 1-12. January is 1.
struct byMonth;

/// A bus station.
struct BusStation;

/// The characteristics of associated patients, such as age, gender, race etc.
struct epidemiology;

/// A trial design in which neither the researcher, the person administering the therapy nor the patient knows the details of the treatment the patient was randomly assigned to.
struct TripleBlindedTrial;

/// Computer programming source code. Example: Full (compile ready) solutions, code snippet samples, scripts, templates.
struct Code;

/// The telephone number.
struct telephone;

/// If applicable, a description of the pathophysiology associated with the anatomical system, including potential abnormal changes in the mechanical, physical, and biochemical functions of the system.
struct associatedPathophysiology;

/// An image of the item. This can be a [[URL]] or a fully described [[ImageObject]].
struct image;

/// A Category code contained in this code set.
struct hasCategoryCode;

/// The number of tracks in this album or playlist.
struct numTracks;

/// A ticket associated with the reservation.
struct reservedTicket;

/// Human-readable terms of service documentation.
struct termsOfService;

/// The URL for a feed, e.g. associated with a podcast series, blog, or series of date-stamped updates. This is usually RSS or Atom.
struct webFeed;

/// A file containing a note, primarily for the author.
struct NoteDigitalDocument;

/// The act of expressing a desire about the object. An agent wants an object.
struct WantAction;

/// If responding yes, the number of guests who will attend in addition to the invitee.
struct additionalNumberOfGuests;

/// A lake (for example, Lake Pontrachain).
struct LakeBodyOfWater;

/// Any other drug related to this one, for example commonly-prescribed alternatives.
struct relatedDrug;

/// RefundTypeEnumeration enumerates several kinds of product return refund types.
struct RefundTypeEnumeration;

/// An event that this event is a part of. For example, a collection of individual music performances might each have a music festival as their superEvent.
struct superEvent;

/// The number of owners of the vehicle, including the current one.\n\nTypical unit code(s): C62
struct numberOfPreviousOwners;

/// Branch of medicine that pertains to the health services to improve and protect community health, especially epidemiology, sanitation, immunization, and preventive medicine.
struct PublicHealth;

/// A description of any sensory requirements and levels necessary to function on the job, including hearing and vision. Defined terms such as those in O*net may be used, but note that there is no way to specify the level of ability as well as its nature when using a defined term.
struct sensoryRequirement;

/// numc19overflowpats - ED/OVERFLOW: Patients with suspected or confirmed COVID-19 who are in the ED or any overflow location awaiting an inpatient bed.
struct cvdNumC19OverflowPats;

/// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF in Spain.
struct taxID;

/// The act of organizing tasks/objects/events by associating resources to it.
struct AllocateAction;

/// A condition the test is used to diagnose.
struct usedToDiagnose;

/// Strength of evidence of the data used to formulate the guideline (enumerated).
struct evidenceLevel;

/// Features or modules provided by this application (and possibly required by other applications).
struct featureList;

/// SingleRelease.
struct SingleRelease;

/// A flag to signal that the item, event, or place is accessible for free.
struct isAccessibleForFree;

/// The movement the muscle generates.
struct muscleAction;

/// Prerequisites for enrolling in the program.
struct programPrerequisites;

/// The key, mode, or scale this composition uses.
struct musicalKey;

/// Indicates a property used as a constraint to define a [[StatisticalPopulation]] with respect to the set of entities
///   corresponding to an indicated type (via [[populationType]]).
struct constrainingProperty;

/// A [[RealEstateListing]] is a listing that describes one or more real-estate [[Offer]]s (whose [[businessFunction]] is typically to lease out, or to sell).
///   The [[RealEstateListing]] type itself represents the overall listing, as manifested in some [[WebPage]].
///   
struct RealEstateListing;

/// If the file can be downloaded, URL to download the binary.
struct downloadUrl;

/// The number of milligrams of cholesterol.
struct cholesterolContent;

/// True if the drug is available in a generic form (regardless of name).
struct isAvailableGenerically;

/// The offer price of a product, or of a price component when attached to PriceSpecification and its subtypes.\n\nUsage guidelines:\n\n* Use the [[priceCurrency]] property (with standard formats: [ISO 4217 currency format](http://en.wikipedia.org/wiki/ISO_4217) e.g. "USD"; [Ticker symbol](https://en.wikipedia.org/wiki/List_of_cryptocurrencies) for cryptocurrencies e.g. "BTC"; well known names for [Local Exchange Tradings Systems](https://en.wikipedia.org/wiki/Local_exchange_trading_system) (LETS) and other currency types e.g. "Ithaca HOUR") instead of including [ambiguous symbols](http://en.wikipedia.org/wiki/Dollar_sign#Currencies_that_use_the_dollar_or_peso_sign) such as '$' in the value.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.\n* Note that both [RDFa](http://www.w3.org/TR/xhtml-rdfa-primer/#using-the-content-attribute) and Microdata syntax allow the use of a "content=" attribute for publishing simple machine-readable values alongside more human-friendly formatting.\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.
///       
struct price;

/// The geographic area where the service is provided.
struct serviceArea;

/// The number of credits or units a full-time student would be expected to take in 1 term however 'term' is defined by the institution.
struct typicalCreditsPerTerm;

/// Specifics about the trial design (enumerated).
struct trialDesign;

/// Individual comic issues are serially published as
///     	part of a larger series. For the sake of consistency, even one-shot issues
///     	belong to a series comprised of a single issue. All comic issues can be
///     	uniquely identified by: the combination of the name and volume number of the
///     	series to which the issue belongs; the issue number; and the variant
///     	description of the issue (if any).
struct ComicIssue;

/// Indicates an item or CreativeWork that this item, or CreativeWork (in some sense), is part of.
struct isPartOf;

/// A person attending the event.
struct attendees;

/// The business function (e.g. sell, lease, repair, dispose) of the offer or component of a bundle (TypeAndQuantityNode). The default is http://purl.org/goodrelations/v1#Sell.
struct businessFunction;

/// A plumbing service.
struct Plumber;

/// File size in (mega/kilo) bytes.
struct contentSize;

/// Nationality of the person.
struct nationality;

/// The most generic type of item.
struct Thing;

/// Any medical imaging modality typically used for diagnostic purposes.
struct ImagingTest;

/// The bitrate of the media object.
struct bitrate;

/// A unique instance of a television BroadcastService on a CableOrSatelliteService lineup.
struct TelevisionChannel;

/// A subclass of OrganizationRole used to describe employee relationships.
struct EmployeeRole;

/// The level in terms of progression through an educational or training context. Examples of educational levels include 'beginner', 'intermediate' or 'advanced', and formal sets of level indicators.
struct educationalLevel;

/// Indicates an occurence of a [[Claim]] in some [[CreativeWork]].
struct appearance;

/// Typical progression and happenings of life course of the topic.
struct PrognosisHealthAspect;

/// A sub property of participant. The participant who is at the receiving end of the action.
struct recipient;

/// A pond.
struct Pond;

/// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.\n\nSee also [[eligibleRegion]].
///       
struct ineligibleRegion;

/// A system of medicine based on the principle that a disease can be cured by a substance that produces similar symptoms in healthy people.
struct Homeopathic;

/// Event that this person is a performer or participant in.
struct performerIn;

/// The version of the CreativeWork embodied by a specified resource.
struct version;

/// An option available on this contact point (e.g. a toll-free number or support for hearing-impaired callers).
struct contactOption;

/// The type of service required to have access to the channel (e.g. Standard or Premium).
struct broadcastServiceTier;

/// The day of the week between Thursday and Saturday.
struct Friday;

/// A high school.
struct HighSchool;

/// The publishingPrinciples property indicates (typically via [[URL]]) a document describing the editorial principles of an [[Organization]] (or individual e.g. a [[Person]] writing a blog) that relate to their activities as a publisher, e.g. ethics or diversity policies. When applied to a [[CreativeWork]] (e.g. [[NewsArticle]]) the principles are those of the party primarily responsible for the creation of the [[CreativeWork]].
/// 
/// While such policies are most typically expressed in natural language, sometimes related information (e.g. indicating a [[funder]]) can be expressed using schema.org terminology.
struct publishingPrinciples;

/// A string or text indicating the unit of measurement. Useful if you cannot provide a standard unit code for
/// <a href='unitCode'>unitCode</a>.
struct unitText;

/// Event type: Comedy event.
struct ComedyEvent;

/// Represents EU Energy Efficiency Class G as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryG;

/// The time it takes to actually cook the dish, in [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601).
struct cookTime;

/// Indicates whether this game is multi-player, co-op or single-player.
struct GamePlayMode;

/// A school.
struct School;

/// Label to match an [[OfferShippingDetails]] with a [[DeliveryTimeSettings]] (within the context of a [[shippingSettingsLink]] cross-reference).
struct transitTimeLabel;

/// The act of obtaining an object under an agreement to return it at a later date. Reciprocal of LendAction.\n\nRelated actions:\n\n* [[LendAction]]: Reciprocal of BorrowAction.
struct BorrowAction;

/// The act of transferring money from one place to another place. This may occur electronically or physically.
struct MoneyTransfer;

/// The act of accomplishing something via previous efforts. It is an instantaneous action rather than an ongoing process.
struct AchieveAction;

/// Enumerates different price components that together make up the total price for an offered product.
struct PriceComponentTypeEnumeration;

/// The observedNode of an [[Observation]], often a [[StatisticalPopulation]].
struct observedNode;

/// The day of the week between Monday and Wednesday.
struct Tuesday;

/// A sub property of object. The options subject to this action.
struct actionOption;

/// Book format: Audiobook. This is an enumerated value for use with the bookFormat property. There is also a type 'Audiobook' in the bib extension which includes Audiobook specific properties.
struct AudiobookFormat;

/// Information about how or where to find a topic. Also may contain location data that can be used for where to look for help if the topic is observed.
struct HowOrWhereHealthAspect;

/// The act of achieving victory in a competitive activity.
struct WinAction;

/// The website to access the service.
struct serviceUrl;

/// BusinessSupport: this is a benefit for supporting businesses.
struct BusinessSupport;

/// The media network(s) whose content is broadcast on this station.
struct broadcastAffiliateOf;

/// This ordering relation for qualitative values indicates that the subject is lesser than the object.
struct lesser;

/// The act of producing a visual/graphical representation of an object, typically with a pen/pencil and paper as instruments.
struct DrawAction;

/// Data type: Number.\n\nUsage guidelines:\n\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
struct Number;

/// A resource from which this work is derived or from which it is a modification or adaption.
struct isBasedOn;

/// Description of the meals that will be provided or available for purchase.
struct mealService;

/// Indicates a legal jurisdiction, e.g. of some legislation, or where some government service is based.
struct jurisdiction;

/// Book format: Paperback.
struct Paperback;

/// The neurological pathway extension that inputs and sends information to the brain or spinal cord.
struct sensoryUnit;

/// The identifier of the order item.
struct orderItemNumber;

/// Relates a property to a property that is its inverse. Inverse properties relate the same pairs of items to each other, but in reversed direction. For example, the 'alumni' and 'alumniOf' properties are inverseOf each other. Some properties don't have explicit inverses; in these situations RDFa and JSON-LD syntax for reverse properties can be used.
struct inverseOf;

/// A [[Claim]] in Schema.org represents a specific, factually-oriented claim that could be the [[itemReviewed]] in a [[ClaimReview]]. The content of a claim can be summarized with the [[text]] property. Variations on well known claims can have their common identity indicated via [[sameAs]] links, and summarized with a [[name]]. Ideally, a [[Claim]] description includes enough contextual information to minimize the risk of ambiguity or inclarity. In practice, many claims are better understood in the context in which they appear or the interpretations provided by claim reviews.
/// 
///   Beyond [[ClaimReview]], the Claim type can be associated with related creative works - for example a [[ScholaryArticle]] or [[Question]] might be [[about]] some [[Claim]].
/// 
///   At this time, Schema.org does not define any types of relationship between claims. This is a natural area for future exploration.
///   
struct Claim;

/// Multicellular parasite that causes an infection.
struct MulticellularParasite;

///  A point value or interval for product characteristics and other purposes.
struct QuantitativeValue;

/// A subgrouping of the menu (by dishes, course, serving time period, etc.).
struct hasMenuSection;

/// Software application help.
struct softwareHelp;

/// The drive wheel configuration, i.e. which roadwheels will receive torque from the vehicle's engine via the drivetrain.
struct driveWheelConfiguration;

/// 'merchant' is an out-dated term for 'seller'.
struct merchant;

/// A medical test performed by a laboratory that typically involves examination of a tissue sample by a pathologist.
struct PathologyTest;

/// A [callsign](https://en.wikipedia.org/wiki/Call_sign), as used in broadcasting and radio communications to identify people, radio and TV stations, or vehicles.
struct callSign;

/// A DefinedRegion is a geographic area defined by potentially arbitrary (rather than political, administrative or natural geographical) criteria. Properties are provided for defining a region by reference to sets of postal codes.
/// 
/// Examples: a delivery destination when shopping. Region where regional pricing is configured.
/// 
/// Requirement 1:
/// Country: US
/// States: "NY", "CA"
/// 
/// Requirement 2:
/// Country: US
/// PostalCode Set: { [94000-94585], [97000, 97999], [13000, 13599]}
/// { [12345, 12345], [78945, 78945], }
/// Region = state, canton, prefecture, autonomous community...
struct DefinedRegion;

/// A pointer from a newer variant of a product  to its previous, often discontinued predecessor.
struct successorOf;

/// An entity holding detailed information about the available bed types, e.g. the quantity of twin beds for a hotel room. For the single case of just one bed of a certain type, you can use bed directly with a text. See also [[BedType]] (under development).
struct BedDetails;

/// Book format: GraphicNovel. May represent a bound collection of ComicIssue instances.
struct GraphicNovel;

/// An actor, e.g. in tv, radio, movie, video games etc., or in an event. Actors can be associated with individual items or with a series, episode, clip.
struct actor;

/// The serving size, in terms of the number of volume or mass.
struct servingSize;

/// The Event where the CreativeWork was recorded. The CreativeWork may capture all or part of the event.
struct recordedAt;

/// The most generic type of entity related to health and the practice of medicine.
struct MedicalEntity;

/// Nonprofit501c16: Non-profit type referring to Cooperative Organizations to Finance Crop Operations.
struct Nonprofit501c16;

/// A statement of the money due for goods or services; a bill.
struct Invoice;

/// Are in-store returns offered?
struct inStoreReturnsOffered;

/// A series of movies. Included movies can be indicated with the hasPart property.
struct MovieSeries;

/// Code used to redeem a discount.
struct discountCode;

/// The stop or station from which the bus arrives.
struct arrivalBusStop;

/// The organization (airline, travelers' club, etc.) the membership is made with.
struct hostingOrganization;

/// Physical activity of relatively low intensity that depends primarily on the aerobic energy-generating process; during activity, the aerobic metabolism uses oxygen to adequately meet energy demands during exercise.
struct AerobicActivity;

/// The act of inserting at the end if an ordered collection.
struct AppendAction;

/// A sub property of instrument. The exercise plan used on this action.
struct exercisePlan;

/// Approximate or typical time it takes to work with or through this learning resource for the typical intended target audience, e.g. 'PT30M', 'PT1H25M'.
struct timeRequired;

/// Describes a reservation for travel, dining or an event. Some reservations require tickets. \n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, restaurant reservations, flights, or rental cars, use [[Offer]].
struct Reservation;

/// A fact-checking review of claims made (or reported) in some creative work (referenced via itemReviewed).
struct ClaimReview;

/// The (e.g. fictional) character, Person or Organization to whom the quotation is attributed within the containing CreativeWork.
struct spokenByCharacter;

/// A member of a music group&#x2014;for example, John, Paul, George, or Ringo.
struct musicGroupMember;

/// The scope of the warranty promise.
struct warrantyScope;

/// The GTIN-13 code of the product, or the product to which the offer refers. This is equivalent to 13-digit ISBN codes and EAN UCC-13. Former 12-digit UPC codes can be converted into a GTIN-13 code by simply adding a preceding zero. See [GS1 GTIN Summary](http://www.gs1.org/barcodes/technical/idkeys/gtin) for more details.
struct gtin13;

/// The business function specifies the type of activity or access (i.e., the bundle of rights) offered by the organization or business person through the offer. Typical are sell, rental or lease, maintenance or repair, manufacture / produce, recycle / dispose, engineering / construction, or installation. Proprietary specifications of access rights are also instances of this class.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#ConstructionInstallation\n* http://purl.org/goodrelations/v1#Dispose\n* http://purl.org/goodrelations/v1#LeaseOut\n* http://purl.org/goodrelations/v1#Maintain\n* http://purl.org/goodrelations/v1#ProvideService\n* http://purl.org/goodrelations/v1#Repair\n* http://purl.org/goodrelations/v1#Sell\n* http://purl.org/goodrelations/v1#Buy
///         
struct BusinessFunction;

/// A type of medical procedure that involves percutaneous techniques, where access to organs or tissue is achieved via needle-puncture of the skin. For example, catheter-based procedures like stent delivery.
struct PercutaneousProcedure;

/// A business providing entertainment.
struct EntertainmentBusiness;

/// Properties that take Energy as values are of the form '&lt;Number&gt; &lt;Energy unit of measure&gt;'.
struct Energy;

/// OrderStatus representing availability of an order for pickup.
struct OrderPickupAvailable;

/// The act of asking someone to attend an event. Reciprocal of RsvpAction.
struct InviteAction;

/// For a [[NewsMediaOrganization]] or other news-related [[Organization]], a statement explaining when authors of articles are not named in bylines.
struct noBylinesPolicy;

/// A particular physical business or branch of an organization. Examples of LocalBusiness include a restaurant, a particular branch of a restaurant chain, a branch of a bank, a medical practice, a club, a bowling alley, etc.
struct LocalBusiness;

/// An article, such as a news article or piece of investigative report. Newspapers and magazines have articles of many different types and this is intended to cover them all.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
struct Article;

/// A ski resort.
struct SkiResort;

/// A subject of the study, i.e. one of the medical conditions, therapies, devices, drugs, etc. investigated by the study.
struct studySubject;

/// Indicates the relationship type of a Web link. 
struct linkRelationship;

/// A dataset contained in this catalog.
struct dataset;

/// A component of the human body circulatory system comprised of an intricate network of hollow tubes that transport blood throughout the entire body.
struct Vessel;

/// MissingContext: ...
struct MissingContext;

/// A sub property of result. The Comment created or sent as a result of this action.
struct resultComment;

/// The release date of a product or product model. This can be used to distinguish the exact variant of a product.
struct releaseDate;

/// The phone number to use to access the service.
struct servicePhone;

/// The type of screening or video broadcast used (e.g. IMAX, 3D, SD, HD, etc.).
struct videoFormat;

/// The category or type of credential being described, for example "degree”, “certificate”, “badge”, or more specific term.
struct credentialCategory;

/// Target Operating System / Product to which the code applies.  If applies to several versions, just the product name can be used.
struct targetProduct;

/// The distance between the centers of the front and rear wheels.\n\nTypical unit code(s): CMT for centimeters, MTR for meters, INH for inches, FOT for foot/feet
struct wheelbase;

/// A page providing information on how to book a tour of some [[Place]], such as an [[Accommodation]] or [[ApartmentComplex]] in a real estate setting, as well as other kinds of tours as appropriate.
struct tourBookingPage;

/// A state or province of a country.
struct State;

/// Permission(s) required to run the app (for example, a mobile app may require full internet access or may run only on wifi).
struct permissions;

/// Whether the 3DModel allows resizing. For example, room layout applications often do not allow 3DModel elements to be resized to reflect reality.
struct isResizable;

/// A casino.
struct Casino;

/// For an [[Organization]] (typically a [[NewsMediaOrganization]]), a statement about policy on use of unnamed sources and the decision process required.
struct unnamedSourcesPolicy;

/// Lung and respiratory system clinical examination.
struct Lung;

/// A construction business.\n\nA HomeAndConstructionBusiness is a [[LocalBusiness]] that provides services around homes and buildings.\n\nAs a [[LocalBusiness]] it can be described as a [[provider]] of one or more [[Service]]\(s).
struct HomeAndConstructionBusiness;

/// A legal document such as an act, decree, bill, etc. (enforceable or not) or a component of a legal act (like an article).
struct Legislation;

/// An [[Article]] that an external entity has paid to place or to produce to its specifications. Includes [advertorials](https://en.wikipedia.org/wiki/Advertorial), sponsored content, native advertising and other paid content.
struct AdvertiserContentArticle;

/// Radiography is an imaging technique that uses electromagnetic radiation other than visible light, especially X-rays, to view the internal structure of a non-uniformly composed and opaque object such as the human body.
struct Radiography;

/// A specific payment status. For example, PaymentDue, PaymentComplete, etc.
struct PaymentStatusType;

/// A photograph of this place.
struct photo;

/// A sign detected by the test.
struct signDetected;

/// The act of editing a recipient by removing one of its objects.
struct DeleteAction;

/// The power of the vehicle's engine.
///     Typical unit code(s): KWT for kilowatt, BHP for brake horsepower, N12 for metric horsepower (PS, with 1 PS = 735,49875 W)\n\n* Note 1: There are many different ways of measuring an engine's power. For an overview, see  [http://en.wikipedia.org/wiki/Horsepower#Engine_power_test_codes](http://en.wikipedia.org/wiki/Horsepower#Engine_power_test_codes).\n* Note 2: You can link to information about how the given value has been determined using the [[valueReference]] property.\n* Note 3: You can use [[minValue]] and [[maxValue]] to indicate ranges.
struct enginePower;

/// The actual body of the article.
struct articleBody;

/// An action that has already taken place.
struct CompletedActionStatus;

/// Reserving a concrete object.\n\nRelated actions:\n\n* [[ScheduleAction]]: Unlike ScheduleAction, ReserveAction reserves concrete objects (e.g. a table, a hotel) towards a time slot / spatial allocation.
struct ReserveAction;

/// The name given to how bone physically connects to each other.
struct structuralClass;

/// An episode of a TV/radio series or season.
struct episodes;

/// A colleague of the person.
struct colleagues;

/// The anatomical or organ system drained by this vessel; generally refers to a specific part of an organ.
struct regionDrained;

/// A secondary title of the CreativeWork.
struct alternativeHeadline;

/// e.g. Painting, Drawing, Sculpture, Print, Photograph, Assemblage, Collage, etc.
struct artform;

/// A position played, performed or filled by a person or organization, as part of an organization. For example, an athlete in a SportsTeam might play in the position named 'Quarterback'.
struct namedPosition;

/// RestockingFees ...
struct RestockingFees;

/// The status of an Action.
struct ActionStatusType;

/// Indicates the design and body style of the vehicle (e.g. station wagon, hatchback, etc.).
struct bodyType;

/// Web page type: Collection page.
struct CollectionPage;

/// When the item is available for pickup from the store, locker, etc.
struct availableFrom;

/// The type of bed or beds included in the accommodation. For the single case of just one bed of a certain type, you use bed directly with a text.
///       If you want to indicate the quantity of a certain kind of bed, use an instance of BedDetails. For more detailed information, use the amenityFeature property.
struct bed;

/// A Catholic church.
struct CatholicChurch;

/// An agent associated with the publication event.
struct publishedBy;

/// The type of bed to which the BedDetail refers, i.e. the type of bed available in the quantity indicated by quantity.
struct typeOfBed;

/// Indicates the date on which the current structured data was generated / published. Typically used alongside [[sdPublisher]]
struct sdDatePublished;

/// A type of financial product that typically requires the client to transfer funds to a financial service in return for potential beneficial financial return.
struct InvestmentOrDeposit;

/// A diet conforming to Jewish dietary practices.
struct KosherDiet;

/// A category of alignment between the learning resource and the framework node. Recommended values include: 'requires', 'textComplexity', 'readingLevel', and 'educationalSubject'.
struct alignmentType;

/// A media season e.g. tv, radio, video game etc.
struct CreativeWorkSeason;

/// A specific branch of medical science that pertains to diagnosis and treatment of disorders of digestive system.
struct Gastroenterologic;

/// Physical address of the item.
struct address;

/// Instances of the class [[Observation]] are used to specify observations about an entity (which may or may not be an instance of a [[StatisticalPopulation]]), at a particular time. The principal properties of an [[Observation]] are [[observedNode]], [[measuredProperty]], [[measuredValue]] (or [[median]], etc.) and [[observationDate]] ([[measuredProperty]] properties can, but need not always, be W3C RDF Data Cube "measure properties", as in the [lifeExpectancy example](https://www.w3.org/TR/vocab-data-cube/#dsd-example)).
/// See also [[StatisticalPopulation]], and the [data and datasets](/docs/data-and-datasets.html) overview for more details.
///   
struct Observation;

/// An arrangement derived from the composition.
struct musicArrangement;

/// The framework to which the resource being described is aligned.
struct educationalFramework;

/// A pet store.
struct PetStore;

/// A link to the page containing the comments of the CreativeWork.
struct discussionUrl;

/// Specifies the minimum allowed range for number of characters in a literal value.
struct valueMinLength;

/// An over the air or online broadcast event.
struct BroadcastEvent;

/// Car repair, sales, or parts.
struct AutomotiveBusiness;

/// The act of resuming a device or application which was formerly paused (e.g. resume music playback or resume a timer).
struct ResumeAction;

/// The area within which users can expect to reach the broadcast service.
struct area;

/// Location in the body of the anatomical structure.
struct bodyLocation;

/// The act of consuming audio content.
struct ListenAction;

/// A short segment/part of a video game.
struct VideoGameClip;

/// The substage, e.g. 'a' for Stage IIIa.
struct subStageSuffix;

/// Represents the distance fee (e.g., price per km or mile) part of the total price for an offered product, for example a car rental
struct DistanceFee;

/// The act of giving money in return for temporary use, but not ownership, of an object such as a vehicle or property. For example, an agent rents a property from a landlord in exchange for a periodic payment.
struct RentAction;

/// A defined range of postal codes indicated by a common textual prefix. Used for non-numeric systems such as UK.
struct postalCodePrefix;

/// A person assigned to instruct or provide instructional assistance for the [[CourseInstance]].
struct instructor;

/// Amount of mortgage mandate that can be converted into a proper mortgage at a later stage.
struct loanMortgageMandateAmount;

/// Indicates the [[productGroupID]] for a [[ProductGroup]] that this product [[isVariantOf]]. 
struct inProductGroupWithID;

/// A director of e.g. tv, radio, movie, video gaming etc. content, or of an event. Directors can be associated with individual items or with a series, episode, clip.
struct director;

/// Cardiovascular system assessment withclinical examination.
struct CardiovascularExam;

/// For an [[Article]], typically a [[NewsArticle]], the backstory property provides a textual summary giving a brief explanation of why and how an article was created. In a journalistic setting this could include information about reporting process, methods, interviews, data sources, etc.
struct backstory;

/// Classification of the album by it's type of content: soundtrack, live album, studio album, etc.
struct albumProductionType;

/// The medical care by a physician, or other health-care professional, who is the patient's first contact with the health-care system and who may recommend a specialist if necessary.
struct PrimaryCare;

/// The composer of the soundtrack.
struct musicBy;

/// A system of medicine based on common theoretical concepts that originated in China and evolved over thousands of years, that uses herbs, acupuncture, exercise, massage, dietary therapy, and other methods to treat a wide range of conditions.
struct TraditionalChinese;

/// An image file.
struct ImageObject;

/// The distance travelled, e.g. exercising or travelling.
struct distance;

/// An email message.
struct EmailMessage;

/// ATM/cash machine.
struct AutomatedTeller;

/// Represents the manufacturer suggested retail price ("MSRP") of an offered product.
struct MSRP;

/// Indicates the primary entity described in some page or other CreativeWork.
struct mainEntity;

/// When a rental car can be dropped off.
struct dropoffTime;

/// Type of software application, e.g. 'Game, Multimedia'.
struct applicationCategory;

/// numventuse - MECHANICAL VENTILATORS IN USE: Total number of ventilators in use.
struct cvdNumVentUse;

/// The unit of measurement given using the UN/CEFACT Common Code (3 characters) or a URL. Other codes than the UN/CEFACT Common Code may be used with a prefix followed by a colon.
struct unitCode;

/// [[Recommendation]] is a type of [[Review]] that suggests or proposes something as the best option or best course of action. Recommendations may be for products or services, or other concrete things, as in the case of a ranked list or product guide. A [[Guide]] may list multiple recommendations for different categories. For example, in a [[Guide]] about which TVs to buy, the author may have several [[Recommendation]]s.
struct Recommendation;

/// The most generic kind of creative work, including books, movies, photographs, software programs, etc.
struct CreativeWork;

/// LaserDiscFormat.
struct LaserDiscFormat;

/// Date on which the content on this web page was last reviewed for accuracy and/or completeness.
struct lastReviewed;

/// Indicates some accommodation that this floor plan describes.
struct isPlanForApartment;

/// A sub property of location. The specific food event where the action occurred.
struct foodEvent;

/// A single, identifiable product instance (e.g. a laptop with a particular serial number).
struct IndividualProduct;

/// Specifying the health condition(s) of a patient, medical study, or other target audience.
struct healthCondition;

/// The act of capturing sound and moving images on film, video, or digitally.
struct FilmAction;

/// The type of permission granted the person, organization, or audience.
struct permissionType;

/// Collection, [fonds](https://en.wikipedia.org/wiki/Fonds), or item held, kept or maintained by an [[ArchiveOrganization]].
struct archiveHeld;

/// Something relating to or practicing dermatology
struct Dermatologic;

/// Any physical activity engaged in for recreational purposes. Examples may include ballroom dancing, roller skating, canoeing, fishing, etc.
struct LeisureTimeActivity;

/// The date that this organization was dissolved.
struct dissolutionDate;

/// The duration of the item (movie, audio recording, event, etc.) in [ISO 8601 date format](http://en.wikipedia.org/wiki/ISO_8601).
struct duration;

/// The torque (turning force) of the vehicle's engine.\n\nTypical unit code(s): NU for newton metre (N m), F17 for pound-force per foot, or F48 for pound-force per inch\n\n* Note 1: You can link to information about how the given value has been determined (e.g. reference RPM) using the [[valueReference]] property.\n* Note 2: You can use [[minValue]] and [[maxValue]] to indicate ranges.
struct torque;

/// A Series in schema.org is a group of related items, typically but not necessarily of the same kind. See also [[CreativeWorkSeries]], [[EventSeries]].
struct Series;

/// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
struct location;

/// The earliest date the package may arrive.
struct expectedArrivalFrom;

/// The CreativeWork encoded by this media object.
struct encodesCreativeWork;

/// A CSS selector, e.g. of a [[SpeakableSpecification]] or [[WebPageElement]]. In the latter case, multiple matches within a page can constitute a single conceptual "Web page element".
struct cssSelector;

/// All-wheel Drive is a transmission layout where the engine drives all four wheels.
struct AllWheelDriveConfiguration;

/// Residence type: Gated community.
struct GatedResidenceCommunity;

/// A specific branch of medical science that is concerned with the study of the cause, origin and nature of a disease state, including its consequences as a result of manifestation of the disease. In clinical care, the term is used to designate a branch of medicine using laboratory tests to diagnose and determine the prognostic significance of illness.
struct Pathology;

/// The billing address for the order.
struct billingAddress;

/// A [[NewsArticle]] and [[CriticReview]] providing a professional critic's assessment of a service, product, performance, or artistic or literary work.
struct ReviewNewsArticle;

/// Fictional person connected with a creative work.
struct character;

/// The geo coordinates of the place.
struct geo;

/// A recycling center.
struct RecyclingCenter;

/// An outlet store.
struct OutletStore;

/// A data catalog which contains this dataset.
struct catalog;

/// A diagnostic test that can identify this sign.
struct identifyingTest;

/// The North American Industry Classification System (NAICS) code for a particular organization or business person.
struct naics;

/// The order is being paid as part of the referenced Invoice.
struct partOfInvoice;

/// The legal requirements such as citizenship, visa and other documentation required for an applicant to this job.
struct eligibilityToWorkRequirement;

/// The total integer number of bathrooms in a some [[Accommodation]], following real estate conventions as [documented in RESO](https://ddwiki.reso.org/display/DDW17/BathroomsTotalInteger+Field): "The simple sum of the number of bathrooms. For example for a property with two Full Bathrooms and one Half Bathroom, the Bathrooms Total Integer will be 3.". See also [[numberOfRooms]].
struct numberOfBathroomsTotal;

/// Indicates that the item is damaged.
struct DamagedCondition;

/// A suite in a hotel or other public accommodation, denotes a class of luxury accommodations, the key feature of which is multiple rooms (Source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Suite_(hotel)">http://en.wikipedia.org/wiki/Suite_(hotel)</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct Suite;

/// A reservation for bus travel. \n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
struct BusReservation;

/// A role played, performed or filled by a person or organization. For example, the team of creators for a comic book might fill the roles named 'inker', 'penciller', and 'letterer'; or an athlete in a SportsTeam might play in the position named 'Quarterback'.
struct roleName;

/// The number of the item ordered. If the property is not set, assume the quantity is one.
struct orderQuantity;

/// Medical audience for page.
struct medicalAudience;

/// The hours during which this service or contact is available.
struct hoursAvailable;

/// A resort is a place used for relaxation or recreation, attracting visitors for holidays or vacations. Resorts are places, towns or sometimes commercial establishment operated by a single company (Source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Resort">http://en.wikipedia.org/wiki/Resort</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
///     
struct Resort;

/// A delivery service through which radio content is provided via broadcast over the air or online.
struct RadioBroadcastService;

/// The textual content of this CreativeWork.
struct text;

/// A podcast is an episodic series of digital audio or video files which a user can download and listen to.
struct PodcastSeries;

/// Enrolling participants by invitation only.
struct EnrollingByInvitation;

/// The frequency in MHz for a particular broadcast.
struct broadcastFrequencyValue;

///  The region/country for which this occupational description is appropriate. Note that educational requirements and qualifications can vary between jurisdictions.
struct occupationLocation;

/// A field of public health focusing on improving health characteristics of a defined population in relation with their geographical or environment areas
struct CommunityHealth;

/// Name or unique ID of network. (Networks are often reused across different insurance plans).
struct healthPlanNetworkId;

/// The date when the item is no longer valid.
struct validUntil;

/// Beauty salon.
struct BeautySalon;

/// MerchantReturnEnumeration enumerates several kinds of product return policy. Note that this structure may not capture all aspects of the policy.
struct MerchantReturnEnumeration;

/// Indicates the number of available accommodation units in an [[ApartmentComplex]], or the number of accommodation units for a specific [[FloorPlan]] (within its specific [[ApartmentComplex]]). See also [[numberOfAccommodationUnits]].
struct numberOfAvailableAccommodationUnits;

/// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: they have no point in common. They form a set of disconnected geometries." (a symmetric relationship, as defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM))
struct geoDisjoint;

/// Minimal age of the child.
struct childMinAge;

/// The act of downloading an object.
struct DownloadAction;

/// A specific branch of medical science that deals with benign and malignant tumors, including the study of their development, diagnosis, treatment and prevention.
struct Oncologic;

/// The label that issued the release.
struct recordLabel;

/// A case series (also known as a clinical series) is a medical research study that tracks patients with a known exposure given similar treatment or examines their medical records for exposure and outcome. A case series can be retrospective or prospective and usually involves a smaller number of patients than the more powerful case-control studies or randomized controlled trials. Case series may be consecutive or non-consecutive, depending on whether all cases presenting to the reporting authors over a period of time were included, or only a selection.
struct CaseSeries;

/// Given name. In the U.S., the first name of a Person.
struct givenName;

/// A person or organization can have different contact points, for different purposes. For example, a sales contact point, a PR contact point and so on. This property is used to specify the kind of contact point.
struct contactType;

/// Event type: Visual arts event.
struct VisualArtsEvent;

/// GovernmentBenefitsType enumerates several kinds of government benefits to support the COVID-19 situation. Note that this structure may not capture all benefits offered.
struct GovernmentBenefitsType;

/// Represents the cleaning fee part of the total price for an offered product, for example a vacation rental
struct CleaningFee;

/// The type of composition (e.g. overture, sonata, symphony, etc.).
struct musicCompositionForm;

/// Data type: Integer.
struct Integer;

/// Information about school closures.
struct schoolClosuresInfo;

/// The type of the legislation. Examples of values are "law", "act", "directive", "decree", "regulation", "statutory instrument", "loi organique", "règlement grand-ducal", etc., depending on the country.
struct legislationType;

/// This ordering relation for qualitative values indicates that the subject is greater than or equal to the object.
struct greaterOrEqual;

/// The LearningResource type can be used to indicate [[CreativeWork]]s (whether physical or digital) that have a particular and explicit orientation towards learning, education, skill acquisition, and other educational purposes.
/// 
/// [[LearningResource]] is expected to be used as an addition to a primary type such as [[Book]], [[Video]], [[Product]] etc.
/// 
/// [[EducationEvent]] serves a similar purpose for event-like things (e.g. a [[Trip]]). A [[LearningResource]] may be created as a result of an [[EducationEvent]], for example by recording one.
struct LearningResource;

/// The gender of the person or audience.
struct suggestedGender;

/// The current price of a currency.
struct currentExchangeRate;

/// The act of finding an object.\n\nRelated actions:\n\n* [[SearchAction]]: FindAction is generally lead by a SearchAction, but not necessarily.
struct FindAction;

/// Number of times one should repeat the activity.
struct repetitions;

/// An audio recording of the work.
struct recordedAs;

/// The GTIN-14 code of the product, or the product to which the offer refers. See [GS1 GTIN Summary](http://www.gs1.org/barcodes/technical/idkeys/gtin) for more details.
struct gtin14;

/// The location in which the study is taking/took place.
struct studyLocation;

/// The time needed to accelerate the vehicle from a given start velocity to a given target velocity.\n\nTypical unit code(s): SEC for seconds\n\n* Note: There are unfortunately no standard unit codes for seconds/0..100 km/h or seconds/0..60 mph. Simply use "SEC" for seconds and indicate the velocities in the [[name]] of the [[QuantitativeValue]], or use [[valueReference]] with a [[QuantitativeValue]] of 0..60 mph or 0..100 km/h to specify the reference speeds.
struct accelerationTime;

/// The medium or means of delivery of the course instance or the mode of study, either as a text label (e.g. "online", "onsite" or "blended"; "synchronous" or "asynchronous"; "full-time" or "part-time") or as a URL reference to a term from a controlled vocabulary (e.g. https://ceds.ed.gov/element/001311#Asynchronous ).
struct courseMode;

/// Number of full bathrooms - The total number of full and ¾ bathrooms in an [[Accommodation]]. This corresponds to the [BathroomsFull field in RESO](https://ddwiki.reso.org/display/DDW17/BathroomsFull+Field).
struct numberOfFullBathrooms;

/// First postal code in a range (included).
struct postalCodeBegin;

/// The unique identifier for the bus.
struct busNumber;

/// A radio channel that uses AM.
struct AMRadioChannel;

/// An order item is a line of an order. It includes the quantity and shipping details of a bought offer.
struct OrderItem;

/// A possible unexpected and unfavorable evolution of a medical condition. Complications may include worsening of the signs or symptoms of the disease, extension of the condition to other organ systems, etc.
struct possibleComplication;

/// The category of cost, such as wholesale, retail, reimbursement cap, etc.
struct costCategory;

/// Runtime platform or script interpreter dependencies (Example - Java v1, Python2.3, .Net Framework 3.0).
struct runtime;

/// The year an [[Accommodation]] was constructed. This corresponds to the [YearBuilt field in RESO](https://ddwiki.reso.org/display/DDW17/YearBuilt+Field). 
struct yearBuilt;

/// Game server status: OfflineTemporarily. Server is offline now but it can be online soon.
struct OfflineTemporarily;

/// The underlying innervation associated with the muscle.
struct nerve;

/// Place of worship, such as a church, synagogue, or mosque.
struct PlaceOfWorship;

/// The drug's cost represents the wholesale acquisition cost of the drug.
struct Wholesale;

/// An entry point, within some Web-based protocol.
struct EntryPoint;

/// The service through with the permit was granted.
struct issuedThrough;

/// A type of blood vessel that specifically carries lymph fluid unidirectionally toward the heart.
struct LymphaticVessel;

/// Structured values are used when the value of a property has a more complex structure than simply being a textual value or a reference to another thing.
struct StructuredValue;

/// The actual body of the review.
struct reviewBody;

/// Organization offering the job position.
struct hiringOrganization;

/// Number of people the reservation should accommodate.
struct partySize;

/// The date the invoice is scheduled to be paid.
struct scheduledPaymentDate;

/// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
struct amenityFeature;

/// Specifying a drug or medicine used in a medication procedure
struct drug;

/// The supported encoding type(s) for an EntryPoint request.
struct encodingType;

/// The act of participating in exertive activity for the purposes of improving health and fitness.
struct ExerciseAction;

/// A person or organization that supports (sponsors) something through some kind of financial contribution.
struct funder;

/// Countries for which the application is supported. You can also provide the two-letter ISO 3166-1 alpha-2 country code.
struct countriesSupported;

/// The act of consuming written content.
struct ReadAction;

/// A tourist trip. A created itinerary of visits to one or more places of interest ([[TouristAttraction]]/[[TouristDestination]]) often linked by a similar theme, geographic area, or interest to a particular [[touristType]]. The [UNWTO](http://www2.unwto.org/) defines tourism trip as the Trip taken by visitors.
///   (See examples below).
struct TouristTrip;

/// A pointer to products or services offered by the organization or person.
struct makesOffer;

/// A product provided to consumers and businesses by financial institutions such as banks, insurance companies, brokerage firms, consumer finance companies, and investment companies which comprise the financial services industry.
struct FinancialProduct;

/// Printed music, as opposed to performed or recorded music.
struct SheetMusic;

/// This property specifies the minimal quantity and rounding increment that will be the basis for the billing. The unit of measurement is specified by the unitCode property.
struct billingIncrement;

/// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
struct supersededBy;

/// This ordering relation for qualitative values indicates that the subject is equal to the object.
struct equal;

/// Indicates information about the shipping policies and options associated with an [[Offer]].
struct shippingDetails;

/// Whether the terms for payment of interest can be renegotiated during the life of the loan.
struct renegotiableLoan;

/// Whether the coinsurance applies before or after deductible, etc. TODO: Is this a closed set?
struct healthPlanCoinsuranceOption;

/// The category or type of pharmacy associated with this cost sharing.
struct healthPlanPharmacyCategory;

/// Nonprofit501c13: Non-profit type referring to Cemetery Companies.
struct Nonprofit501c13;

/// Defines the frequency at which [[Events]] will occur according to a schedule [[Schedule]]. The intervals between
///       events should be defined as a [[Duration]] of time.
struct repeatFrequency;

/// An associated logo.
struct logo;

/// The vasculature the lymphatic structure runs, or efferents, to.
struct runsTo;

/// A clothing store.
struct ClothingStore;

/// The number of calories.
struct calories;

/// The branches that comprise the arterial structure.
struct arterialBranch;

/// Typical or recommended followup care after the procedure is performed.
struct followup;

/// A date value in [ISO 8601 date format](http://en.wikipedia.org/wiki/ISO_8601).
struct Date;

/// The amount of fuel consumed for traveling a particular distance or temporal duration with the given vehicle (e.g. liters per 100 km).\n\n* Note 1: There are unfortunately no standard unit codes for liters per 100 km.  Use [[unitText]] to indicate the unit of measurement, e.g. L/100 km.\n* Note 2: There are two ways of indicating the fuel consumption, [[fuelConsumption]] (e.g. 8 liters per 100 km) and [[fuelEfficiency]] (e.g. 30 miles per gallon). They are reciprocal.\n* Note 3: Often, the absolute value is useful only when related to driving speed ("at 80 km/h") or usage pattern ("city traffic"). You can use [[valueReference]] to link the value for the fuel consumption to another value.
struct fuelConsumption;

/// Properties that take Mass as values are of the form '&lt;Number&gt; &lt;Mass unit of measure&gt;'. E.g., '7 kg'.
struct Mass;

/// Represents the invoice price of an offered product.
struct InvoicePrice;

/// Front-wheel drive is a transmission layout where the engine drives the front wheels.
struct FrontWheelDriveConfiguration;

/// Indicates the kind of Map, from the MapCategoryType Enumeration.
struct mapType;

/// A specific question - e.g. from a user seeking answers online, or collected in a Frequently Asked Questions (FAQ) document.
struct Question;

/// A grocery store.
struct GroceryStore;

/// A set of characteristics belonging to people, e.g. who compose an item's target audience.
struct PeopleAudience;

/// A possible complication and/or side effect of this therapy. If it is known that an adverse outcome is serious (resulting in death, disability, or permanent damage; requiring hospitalization; or is otherwise life-threatening or requires immediate medical attention), tag it as a seriouseAdverseOutcome instead.
struct adverseOutcome;

/// The act of dressing oneself in clothing.
struct WearAction;

/// Description of bonus and commission compensation aspects of the job.
struct incentiveCompensation;

/// The cost per unit of a medical drug. Note that this type is not meant to represent the price in an offer of a drug for sale; see the Offer type for that. This type will typically be used to tag wholesale or average retail cost of a drug, or maximum reimbursable cost. Costs of medical drugs vary widely depending on how and where they are paid for, so while this type captures some of the variables, costs should be used with caution by consumers of this schema's markup.
struct DrugCost;

/// numc19hopats - HOSPITAL ONSET: Patients hospitalized in an NHSN inpatient care location with onset of suspected or confirmed COVID-19 14 or more days after hospitalization.
struct cvdNumC19HOPats;

/// The temporalCoverage of a CreativeWork indicates the period that the content applies to, i.e. that it describes, either as a DateTime or as a textual string indicating a time period in [ISO 8601 time interval format](https://en.wikipedia.org/wiki/ISO_8601#Time_intervals). In
///       the case of a Dataset it will typically indicate the relevant time period in a precise notation (e.g. for a 2011 census dataset, the year 2011 would be written "2011/2012"). Other forms of content e.g. ScholarlyArticle, Book, TVSeries or TVEpisode may indicate their temporalCoverage in broader terms - textually or via well-known URL.
///       Written works such as books may sometimes have precise temporal coverage too, e.g. a work set in 1939 - 1945 can be indicated in ISO 8601 interval format format via "1939/1945".
/// 
/// Open-ended date ranges can be written with ".." in place of the end date. For example, "2015-11/.." indicates a range beginning in November 2015 and with no specified final date. This is tentative and might be updated in future when ISO 8601 is officially updated.
struct temporalCoverage;

/// MerchantReturnUnspecified: a product return policy is not specified here.
struct MerchantReturnUnspecified;

/// Link to the drug's label details.
struct labelDetails;

/// Categorization and other types related to a topic.
struct TypesHealthAspect;

/// A code that identifies this [[DefinedTerm]] within a [[DefinedTermSet]]
struct termCode;

/// The International Standard Serial Number (ISSN) that identifies this serial publication. You can repeat this property to identify different formats of, or the linking ISSN (ISSN-L) for, this serial publication.
struct issn;

/// A park.
struct Park;

/// A step-by-step or full explanation about Answer. Can outline how this Answer was achieved or contain more broad clarification or statement about it. 
struct answerExplanation;

/// A church.
struct Church;

/// A sequential publication of comic stories under a
///     	unifying title, for example "The Amazing Spider-Man" or "Groo the
///     	Wanderer".
struct ComicSeries;

/// A resource that was used in the creation of this resource. This term can be repeated for multiple sources. For example, http://example.com/great-multiplication-intro.html.
struct isBasedOnUrl;

/// An answer (possibly one of several, possibly incorrect) to a Question, e.g. on a Question/Answer site.
struct suggestedAnswer;

/// A diet exclusive of gluten.
struct GlutenFreeDiet;

/// Indicates that this legislation (or part of legislation) fulfills the objectives set by another legislation, by passing appropriate implementation measures. Typically, some legislations of European Union's member states or regions transpose European Directives. This indicates a legally binding link between the 2 legislations.
struct legislationTransposes;

/// An list item, e.g. a step in a checklist or how-to description.
struct ListItem;

/// The type of security screening the passenger is subject to.
struct securityScreening;

/// The transaction volume, in a monetary unit, for which the offer or price specification is valid, e.g. for indicating a minimal purchasing volume, to express free shipping above a certain order volume, or to limit the acceptance of credit cards to purchases to a certain minimal amount.
struct eligibleTransactionVolume;

/// The actual infectious agent, such as a specific bacterium.
struct infectiousAgent;

/// Actual bytes of the media object, for example the image file or video file.
struct contentUrl;

/// Information about public transport closures.
struct publicTransportClosuresInfo;

/// Information about travel bans, e.g. in the context of a pandemic.
struct travelBans;

/// An [[OfferForLease]] in Schema.org represents an [[Offer]] to lease out something, i.e. an [[Offer]] whose
///   [[businessFunction]] is [lease out](http://purl.org/goodrelations/v1#LeaseOut.). See [Good Relations](https://en.wikipedia.org/wiki/GoodRelations) for
///   background on the underlying concepts.
///   
struct OfferForLease;

/// A musical group, such as a band, an orchestra, or a choir. Can also be a solo musician.
struct MusicGroup;

/// A pharmacy or drugstore.
struct Pharmacy;

/// The trailer of a movie or tv/radio series, season, episode, etc.
struct trailer;

/// A member of an Organization or a ProgramMembership. Organizations can be members of organizations; ProgramMembership is typically for individuals.
struct member;

/// A listing that describes a job opening in a certain organization.
struct JobPosting;

/// The high level platform(s) where the Action can be performed for the given URL. To specify a specific application or operating system instance, use actionApplication.
struct actionPlatform;

/// Nonprofit501q: Non-profit type referring to Credit Counseling Organizations.
struct Nonprofit501q;

/// Date when this media object was uploaded to this site.
struct uploadDate;

/// numc19hosppats - HOSPITALIZED: Patients currently hospitalized in an inpatient care location who have suspected or confirmed COVID-19.
struct cvdNumC19HospPats;

/// Specifies after how much time this price (or price component) becomes valid and billing starts. Can be used, for example, to model a price increase after the first year of a subscription. The unit of measurement is specified by the unitCode property
struct billingStart;

/// A contact point for a person or organization.
struct contactPoints;

/// The total financial value of the person as calculated by subtracting assets from liabilities.
struct netWorth;

/// A process of care involving exercise, changes to diet, fitness routines, and other lifestyle changes aimed at improving a health condition.
struct LifestyleModification;

/// A pointer to the organization or person making the offer.
struct offeredBy;

/// An intangible type to be applied to any archive content, carrying with it a set of properties required to describe archival items and collections.
struct ArchiveComponent;

/// The unique address by which the BroadcastService can be identified in a provider lineup. In US, this is typically a number.
struct broadcastChannelId;

/// The spatialCoverage of a CreativeWork indicates the place(s) which are the focus of the content. It is a subproperty of
///       contentLocation intended primarily for more technical and detailed materials. For example with a Dataset, it indicates
///       areas that the dataset describes: a dataset of New York weather would have spatialCoverage which was the place: the state of New York.
struct spatialCoverage;

/// A point in time recurring on multiple days in the form hh:mm:ss[Z|(+|-)hh:mm] (see [XML schema for details](http://www.w3.org/TR/xmlschema-2/#time)).
struct Time;

/// Animal shelter.
struct AnimalShelter;

/// Identifies a price component (for example, a line item on an invoice), part of the total price for an offer.
struct priceComponentType;

/// The eventAttendanceMode of an event indicates whether it occurs online, offline, or a mix.
struct eventAttendanceMode;

/// A pointer from a previous, often discontinued variant of the product to its newer variant.
struct predecessorOf;

/// Any specific branch of medical science or practice. Medical specialities include clinical specialties that pertain to particular organ systems and their respective disease states, as well as allied health specialties. Enumerated type.
struct MedicalSpecialty;

/// Abdomen clinical examination.
struct Abdomen;

/// A governmental organization or agency.
struct GovernmentOrganization;

/// An employment agency.
struct EmploymentAgency;

/// The tangible thing generated by the service, e.g. a passport, permit, etc.
struct serviceOutput;

/// An application programming interface accessible over Web/Internet technologies.
struct WebAPI;

/// The quantity that results by performing instructions. For example, a paper airplane, 10 personalized candles.
struct yield;

/// Web page type: Profile page.
struct ProfilePage;

/// An estimated salary for a job posting or occupation, based on a variety of variables including, but not limited to industry, job title, and location. Estimated salaries  are often computed by outside organizations rather than the hiring organization, who may not have committed to the estimated value.
struct estimatedSalary;

/// The act of forming a personal connection with someone/something (object) unidirectionally/asymmetrically to get updates polled from.\n\nRelated actions:\n\n* [[BefriendAction]]: Unlike BefriendAction, FollowAction implies that the connection is *not* necessarily reciprocal.\n* [[SubscribeAction]]: Unlike SubscribeAction, FollowAction implies that the follower acts as an active agent constantly/actively polling for updates.\n* [[RegisterAction]]: Unlike RegisterAction, FollowAction implies that the agent is interested in continuing receiving updates from the object.\n* [[JoinAction]]: Unlike JoinAction, FollowAction implies that the agent is interested in getting updates from the object.\n* [[TrackAction]]: Unlike TrackAction, FollowAction refers to the polling of updates of all aspects of animate objects rather than the location of inanimate objects (e.g. you track a package, but you don't follow it).
struct FollowAction;

/// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
struct sameAs;

/// Throat assessment with  clinical examination.
struct Throat;

/// A contraindication for this therapy.
struct contraindication;

/// Represents the downpayment (up-front payment) price component of the total price for an offered product that has additional installment payments
struct Downpayment;

/// The act of capturing still images of objects using a camera.
struct PhotographAction;

/// An aggregate rating of an Organization related to its role as an employer.
struct EmployerAggregateRating;

/// A person (alive, dead, undead, or fictional).
struct Person;

/// A guideline recommendation that is regarded as efficacious and where quality of the data supporting the recommendation is sound.
struct MedicalGuidelineRecommendation;

/// The date that this organization was founded.
struct foundingDate;

/// A patient is any person recipient of health care services.
struct Patient;

/// A nail salon.
struct NailSalon;

/// A city hall.
struct CityHall;

/// If applicable, the organization that officially recognizes this entity as part of its endorsed system of medicine.
struct recognizingAuthority;

/// The expected arrival time.
struct arrivalTime;

/// Indicates a Web page or service by URL, for product return.
struct merchantReturnLink;

/// Type of ordering (e.g. Ascending, Descending, Unordered).
struct itemListOrder;

/// Indicates the usage of the vehicle as a rental car.
struct RentalVehicleUsage;

/// Official rating of a piece of content&#x2014;for example,'MPAA PG-13'.
struct contentRating;

/// A dataset in downloadable form.
struct DataDownload;

/// The act of an agent relocating to a place.\n\nRelated actions:\n\n* [[TransferAction]]: Unlike TransferAction, the subject of the move is a living Person or Organization rather than an inanimate object.
struct MoveAction;

/// A rating is an evaluation on a numeric scale, such as 1 to 5 stars.
struct Rating;

/// The practice of treatment of disease, injury, or deformity by physical methods such as massage, heat treatment, and exercise rather than by drugs or surgery..
struct Physiotherapy;

/// Represents the list price (the price a product is actually advertised for) of an offered product.
struct ListPrice;

/// A code for a medical entity.
struct MedicalCode;

/// Single-celled organism that causes an infection.
struct Protozoa;

/// The period of time after any due date that the borrower has to fulfil its obligations before a default (failure to pay) is deemed to have occurred.
struct gracePeriod;

/// Contact details for further information relevant to this job posting.
struct applicationContact;

/// Residence type: Single-family home.
struct SingleFamilyResidence;

/// The unit in which the drug is measured, e.g. '5 mg tablet'.
struct drugUnit;

/// One of the continents (for example, Europe or Africa).
struct Continent;

/// Form of markup used. eg. [SSML](https://www.w3.org/TR/speech-synthesis11) or [IPA](https://www.wikidata.org/wiki/Property:P898).
struct speechToTextMarkup;

/// Identifier of the NHSN facility that this data record applies to. Use [[cvdFacilityCounty]] to indicate the county. To provide other details, [[healthcareReportingData]] can be used on a [[Hospital]] entry.
struct cvdFacilityId;

/// Identifies a [[Trip]] that is a subTrip of this Trip.  For example Day 1, Day 2, etc. of a multi-day trip.
struct subTrip;

/// The schema.org [[usageInfo]] property indicates further information about a [[CreativeWork]]. This property is applicable both to works that are freely available and to those that require payment or other transactions. It can reference additional information e.g. community expectations on preferred linking and citation conventions, as well as purchasing details. For something that can be commercially licensed, usageInfo can provide detailed, resource-specific information about licensing options.
/// 
/// This property can be used alongside the license property which indicates license(s) applicable to some piece of content. The usageInfo property can provide information about other licensing options, e.g. acquiring commercial usage rights for an image that is also available under non-commercial creative commons licenses.
struct usageInfo;

/// A mountain, like Mount Whitney or Mount Everest.
struct Mountain;

/// Ultrasound imaging.
struct Ultrasound;

/// A government office&#x2014;for example, an IRS or DMV office.
struct GovernmentOffice;

/// Indicates that the event was changed to allow online participation. See [[eventAttendanceMode]] for specifics of whether it is now fully or partially online.
struct EventMovedOnline;

/// Representation of a text [[textValue]] using the specified [[speechToTextMarkup]]. For example the city name of Houston in IPA: /ˈhjuːstən/.
struct phoneticText;

/// An alias for the item.
struct alternateName;

/// The type of a loan or credit.
struct loanType;

/// A randomized trial design.
struct RandomizedTrial;

/// Nonprofit501c28: Non-profit type referring to National Railroad Retirement Investment Trusts.
struct Nonprofit501c28;

/// A software application.
struct SoftwareApplication;

/// A navigation element of the page.
struct SiteNavigationElement;

/// The reference quantity for which a certain price applies, e.g. 1 EUR per 4 kWh of electricity. This property is a replacement for unitOfMeasurement for the advanced cases where the price does not relate to a standard unit.
struct referenceQuantity;

/// The end date and time of the item (in [ISO 8601 date format](http://en.wikipedia.org/wiki/ISO_8601)).
struct endDate;

/// A description of the employer, career opportunities and work environment for this position.
struct employerOverview;

/// Any additional component of the exercise prescription that may need to be articulated to the patient. This may include the order of exercises, the number of repetitions of movement, quantitative distance, progressions over time, etc.
struct additionalVariable;

/// The steering position is on the left side of the vehicle (viewed from the main direction of driving).
struct LeftHandDriving;

/// A unique identifier for the reservation.
struct reservationId;

/// The airport where the flight terminates.
struct arrivalAirport;

/// The place where the person was born.
struct birthPlace;

/// A car rental business.
struct AutoRental;

/// The upper value of some characteristic or property.
struct maxValue;

/// A secure method for consumers to purchase products or services via debit, credit or smartcards by using RFID or NFC technology.
struct contactlessPayment;

/// Description of what changed in this version.
struct releaseNotes;

/// EPRelease.
struct EPRelease;

/// A designation by the US FDA signifying that adequate and well-controlled studies have failed to demonstrate a risk to the fetus in the first trimester of pregnancy (and there is no evidence of risk in later trimesters).
struct FDAcategoryA;

/// An image of a visual machine-readable code such as a barcode or QR code.
struct Barcode;

/// The type or material of the interior of the vehicle (e.g. synthetic fabric, leather, wood, etc.). While most interior types are characterized by the material used, an interior type can also be based on vehicle usage or target audience.
struct vehicleInteriorType;

/// The act of physically/electronically dispatching an object for transfer from an origin to a destination.Related actions:\n\n* [[ReceiveAction]]: The reciprocal of SendAction.\n* [[GiveAction]]: Unlike GiveAction, SendAction does not imply the transfer of ownership (e.g. I can send you my laptop, but I'm not necessarily giving it to you).
struct SendAction;

/// ShippingDeliveryTime provides various pieces of information about delivery times for shipping.
struct ShippingDeliveryTime;

/// A person who founded this organization.
struct founder;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserPlays;

/// The estimated cost of the supply or supplies consumed when performing instructions.
struct estimatedCost;

/// A class of medical drugs, e.g., statins. Classes can represent general pharmacological class, common mechanisms of action, common physiological effects, etc.
struct DrugClass;

/// governmentBenefitsInfo provides information about government benefits associated with a SpecialAnnouncement.
struct governmentBenefitsInfo;

/// An embassy.
struct Embassy;

/// Nonprofit501e: Non-profit type referring to Cooperative Hospital Service Organizations.
struct Nonprofit501e;

/// Season dedicated to radio broadcast and associated online delivery.
struct RadioSeason;

/// Also known as a panel study. A cohort study is a form of longitudinal study used in medicine and social science. It is one type of study design and should be compared with a cross-sectional study.  A cohort is a group of people who share a common characteristic or experience within a defined period (e.g., are born, leave school, lose their job, are exposed to a drug or a vaccine, etc.). The comparison group may be the general population from which the cohort is drawn, or it may be another cohort of persons thought to have had little or no exposure to the substance under investigation, but otherwise similar. Alternatively, subgroups within the cohort may be compared with each other.
struct CohortStudy;

/// A medical test performed on a sample of a patient's blood.
struct BloodTest;

/// The name of a node in an established educational framework.
struct targetName;

/// The day of the week between Saturday and Monday.
struct Sunday;

/// The act of returning to the origin that which was previously received (concrete objects) or taken (ownership).
struct ReturnAction;

/// The name of the credit card or other method of payment for the order.
struct paymentMethod;

/// The anatomy of the underlying organ system or structures associated with this entity.
struct associatedAnatomy;

/// This type covers computer programming languages such as Scheme and Lisp, as well as other language-like computer representations. Natural languages are best represented with the [[Language]] type.
struct ComputerLanguage;

/// A golf course.
struct GolfCourse;

/// The time at which the UserComment was made.
struct commentTime;

/// Any precaution, guidance, contraindication, etc. related to this drug's use during pregnancy.
struct pregnancyWarning;

/// A business entity type is a conceptual entity representing the legal form, the size, the main line of business, the position in the value chain, or any combination thereof, of an organization or business person.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#Business\n* http://purl.org/goodrelations/v1#Enduser\n* http://purl.org/goodrelations/v1#PublicInstitution\n* http://purl.org/goodrelations/v1#Reseller
/// 	  
struct BusinessEntityType;

/// An educationalRole of an EducationalAudience.
struct educationalRole;

/// The airport where the flight originates.
struct departureAirport;

/// A specific branch of medical science that pertains to the health care of women, particularly in the diagnosis and treatment of disorders affecting the female reproductive system.
struct Gynecologic;

/// The act of arriving at a place. An agent arrives at a destination from a fromLocation, optionally with participants.
struct ArriveAction;

/// Fitness-related activity designed for a specific health-related purpose, including defined exercise routines as well as activity prescribed by a clinician.
struct ExercisePlan;

/// A hostel - cheap accommodation, often in shared dormitories.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct Hostel;

/// A description of an action that is supported.
struct PotentialActionStatus;

/// A structured value providing information about when a certain organization or person owned a certain product.
struct OwnershipInfo;

/// Frequency of payments due, i.e. number of months between payments. This is defined as a frequency, i.e. the reciprocal of a period of time.
struct loanPaymentFrequency;

/// Specific physiologic risks associated to the diet plan.
struct risks;

/// The place and time the release was issued, expressed as a PublicationEvent.
struct releasedEvent;

/// An AnalysisNewsArticle is a [[NewsArticle]] that, while based on factual reporting, incorporates the expertise of the author/producer, offering interpretations and conclusions.
struct AnalysisNewsArticle;

/// Defines the day(s) of the week on which a recurring [[Event]] takes place. May be specified using either [[DayOfWeek]], or alternatively [[Text]] conforming to iCal's syntax for byDay recurrence rules
struct byDay;

/// The act of notifying someone that a future event/action is going to happen as expected.\n\nRelated actions:\n\n* [[CancelAction]]: The antonym of ConfirmAction.
struct ConfirmAction;

/// Nonprofit501c14: Non-profit type referring to State-Chartered Credit Unions, Mutual Reserve Funds.
struct Nonprofit501c14;

/// A technique or technology used in a [[Dataset]] (or [[DataDownload]], [[DataCatalog]]),
/// corresponding to the method used for measuring the corresponding variable(s) (described using [[variableMeasured]]). This is oriented towards scientific and scholarly dataset publication but may have broader applicability; it is not intended as a full representation of measurement, but rather as a high level summary for dataset discovery.
/// 
/// For example, if [[variableMeasured]] is: molecule concentration, [[measurementTechnique]] could be: "mass spectrometry" or "nmr spectroscopy" or "colorimetry" or "immunofluorescence".
/// 
/// If the [[variableMeasured]] is "depression rating", the [[measurementTechnique]] could be "Zung Scale" or "HAM-D" or "Beck Depression Inventory".
/// 
/// If there are several [[variableMeasured]] properties recorded for some given data object, use a [[PropertyValue]] for each [[variableMeasured]] and attach the corresponding [[measurementTechnique]].
///       
struct measurementTechnique;

/// The allowed total occupancy for the accommodation in persons (including infants etc). For individual accommodations, this is not necessarily the legal maximum but defines the permitted usage as per the contractual agreement (e.g. a double room used by a single person).
/// Typical unit code(s): C62 for person
struct occupancy;

/// Disclosure about verification and fact-checking processes for a [[NewsMediaOrganization]] or other fact-checking [[Organization]].
struct verificationFactCheckingPolicy;

/// Game server status: Online. Server is available.
struct Online;

/// A trial that takes place at a single center.
struct SingleCenterTrial;

/// A credential awarded to the Person or Organization.
struct hasCredential;

/// A description of the item.
struct description;

/// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See [background notes](/docs/datamodel.html#mainEntityBackground) for details.
struct mainEntityOfPage;

/// The standard for interpreting thePlan ID. The preferred is "HIOS". See the Centers for Medicare & Medicaid Services for more details.
struct usesHealthPlanIdStandard;

/// An offering of the course at a specific time and place or through specific media or mode of study or to a specific section of students.
struct hasCourseInstance;

/// An abstract is a short description that summarizes a [[CreativeWork]].
struct abstract;

/// The act of momentarily pausing a device or application (e.g. pause music playback or pause a timer).
struct SuspendAction;

/// A sub property of recipient. The recipient blind copied on a message.
struct bccRecipient;

/// A work of art that is primarily visual in character.
struct VisualArtwork;

/// The person or organization who produced the work (e.g. music album, movie, tv/radio series etc.).
struct producer;

/// The name of the item.
struct name;

/// The description of a node in an established educational framework.
struct targetDescription;

/// A collection of music tracks in playlist form.
struct MusicPlaylist;

/// Whether multiple values are allowed for the property.  Default is false.
struct multipleValues;

/// The day of the week between Friday and Sunday.
struct Saturday;

/// Head assessment with clinical examination.
struct Head;

/// A store that sells materials useful or necessary for various hobbies.
struct HobbyShop;

/// The tier(s) for this network.
struct healthPlanNetworkTier;

/// Photographs of this place.
struct photos;

/// A car is a wheeled, self-powered motor vehicle used for transportation.
struct Car;

/// A book.
struct Book;

/// The term "story" is any indivisible, re-printable
///     	unit of a comic, including the interior stories, covers, and backmatter. Most
///     	comics have at least two stories: a cover (ComicCoverArt) and an interior story.
struct ComicStory;

/// An intangible item that describes an alignment between a learning resource and a node in an educational framework.
/// 
/// Should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource [[teaches]] or [[assesses]] a competency.
struct AlignmentObject;

/// A CovidTestingFacility is a [[MedicalClinic]] where testing for the COVID-19 Coronavirus
///       disease is available. If the facility is being made available from an established [[Pharmacy]], [[Hotel]], or other
///       non-medical organization, multiple types can be listed. This makes it easier to re-use existing schema.org information
///       about that place e.g. contact info, address, opening hours. Note that in an emergency, such information may not always be reliable.
///       
struct CovidTestingFacility;

/// A condition or factor that serves as a reason to withhold a certain medical therapy. Contraindications can be absolute (there are no reasonable circumstances for undertaking a course of action) or relative (the patient is at higher risk of complications, but that these risks may be outweighed by other considerations or mitigated by other measures).
struct MedicalContraindication;

/// An overdraft is an extension of credit from a lending institution when an account reaches zero. An overdraft allows the individual to continue withdrawing money even if the account has no funds in it. Basically the bank allows people to borrow a set amount of money.
struct accountOverdraftLimit;

/// The year during which the claimed copyright for the CreativeWork was first asserted.
struct copyrightYear;

/// A place offering space for "Recreational Vehicles", Caravans, mobile homes and the like.
struct RVPark;

/// The neurological pathway extension that involves muscle control.
struct nerveMotor;

/// The predominant type or kind characterizing the learning resource. For example, 'presentation', 'handout'.
struct learningResourceType;

/// A pointer to another product (or multiple products) for which this product is an accessory or spare part.
struct isAccessoryOrSparePartFor;

/// The exchange traded instrument associated with a Corporation object. The tickerSymbol is expressed as an exchange and an instrument name separated by a space character. For the exchange component of the tickerSymbol attribute, we recommend using the controlled vocabulary of Market Identifier Codes (MIC) specified in ISO15022.
struct tickerSymbol;

/// Content about the real life experience of patients or people that have lived a similar experience about the topic. May be forums, topics, Q-and-A and related material.
struct PatientExperienceHealthAspect;

/// An agent orders an object/product/service to be delivered/sent.
struct OrderAction;

/// [[HealthTopicContent]] is [[WebContent]] that is about some aspect of a health topic, e.g. a condition, its symptoms or treatments. Such content may be comprised of several parts or sections and use different types of media. Multiple instances of [[WebContent]] (and hence [[HealthTopicContent]]) can be related using [[hasPart]] / [[isPartOf]] where there is some kind of content hierarchy, and their content described with [[about]] and [[mentions]] e.g. building upon the existing [[MedicalCondition]] vocabulary.
///   
struct HealthTopicContent;

/// A collection of items e.g. creative works or products.
struct Collection;

/// The playlist to which this recording belongs.
struct inPlaylist;

/// A room is a distinguishable space within a structure, usually separated from other spaces by interior walls. (Source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Room">http://en.wikipedia.org/wiki/Room</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct Room;

/// The name of a character played in some acting or performing role, i.e. in a PerformanceRole.
struct characterName;

/// Events that are a part of this event. For example, a conference event includes many presentations, each subEvents of the conference.
struct subEvents;

/// Web page type: Contact page.
struct ContactPage;

/// The platform from which the train departs.
struct departurePlatform;

/// The album to which this recording belongs.
struct inAlbum;

/// Information about the causes and main actions that gave rise to the topic.
struct CausesHealthAspect;

/// The general opening hours for a business. Opening hours can be specified as a weekly time range, starting with days, then times per day. Multiple days can be listed with commas ',' separating each day. Day or time ranges are specified using a hyphen '-'.\n\n* Days are specified using the following two-letter combinations: ```Mo```, ```Tu```, ```We```, ```Th```, ```Fr```, ```Sa```, ```Su```.\n* Times are specified using 24:00 format. For example, 3pm is specified as ```15:00```, 10am as ```10:00```. \n* Here is an example: <code>&lt;time itemprop="openingHours" datetime=&quot;Tu,Th 16:00-20:00&quot;&gt;Tuesdays and Thursdays 4-8pm&lt;/time&gt;</code>.\n* If a business is open 7 days a week, then it can be specified as <code>&lt;time itemprop=&quot;openingHours&quot; datetime=&quot;Mo-Su&quot;&gt;Monday through Sunday, all day&lt;/time&gt;</code>.
struct openingHours;

/// Specifies a location feature by providing a structured value representing a feature of an accommodation as a property-value pair of varying degrees of formality.
struct LocationFeatureSpecification;

/// UnincorporatedAssociationCharity: Non-profit type referring to a charitable company that is not incorporated (UK).
struct UnincorporatedAssociationCharity;

/// Reference documentation for application programming interfaces (APIs).
struct APIReference;

/// Specific physiologic benefits associated to the plan.
struct physiologicalBenefits;

/// The required quantity of the item(s).
struct requiredQuantity;

/// The quantity of the materials being described or an expression of the physical space they occupy.
struct materialExtent;

/// The place where a person lives.
struct Residence;

/// UnemploymentSupport: this is a benefit for unemployment support.
struct UnemploymentSupport;

/// Any collection of tests commonly ordered together.
struct MedicalTestPanel;

/// An anatomical system is a group of anatomical structures that work together to perform a certain task. Anatomical systems, such as organ systems, are one organizing principle of anatomy, and can includes circulatory, digestive, endocrine, integumentary, immune, lymphatic, muscular, nervous, reproductive, respiratory, skeletal, urinary, vestibular, and other systems.
struct AnatomicalSystem;

/// The publication format of the book.
struct BookFormatType;

/// A diet focused on reduced sodium intake.
struct LowSaltDiet;

/// A bus (also omnibus or autobus) is a road vehicle designed to carry passengers. Coaches are luxury busses, usually in service for long distance travel.
struct BusOrCoach;

/// A property-value pair representing an additional characteristics of the entitity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.\n\nNote: Publishers should be aware that applications designed to use specific schema.org properties (e.g. https://schema.org/width, https://schema.org/color, https://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
struct additionalProperty;

/// An agent pays a price to a participant.
struct PayAction;

/// Any object used in a medical capacity, such as to diagnose or treat a patient.
struct MedicalDevice;

/// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
struct broker;

/// The thing -- flight, event, restaurant,etc. being reserved.
struct reservationFor;

/// Information about the risk factors and possible complications that may follow a topic.
struct RisksOrComplicationsHealthAspect;

/// The act of adding at a specific location in an ordered collection.
struct InsertAction;

/// The expected progression of the condition if it is not treated and allowed to progress naturally.
struct naturalProgression;

/// Event type: Education event.
struct EducationEvent;

/// A standardized size of a product or creative work, often simplifying richer information into a simple textual string, either through referring to named sizes or (in the case of product markup), by adopting conventional simplifications. Use of QuantitativeValue with a unitCode or unitText can add more structure; in other cases, the /width, /height, /depth and /weight properties may be more applicable. 
struct size;

/// The medical conditions, treatments, etc. that are the subject of the guideline.
struct guidelineSubject;

/// The act of allocating an action/event/task to some destination (someone or something).
struct AssignAction;

/// A sub property of instrument. The recipe/instructions used to perform the action.
struct recipe;

/// An emergency service, such as a fire station or ER.
struct EmergencyService;

/// The organization or person from which the product was acquired.
struct acquiredFrom;

/// The status of a reservation when a request has been sent, but not confirmed.
struct ReservationPending;

/// Any branch of a field in which people typically develop specific expertise, usually after significant study, time, and effort.
struct Specialty;

/// Content about the benefits and advantages of usage or utilization of topic.
struct BenefitsHealthAspect;

/// Studies carried out on pre-existing data (usually from 'snapshot' surveys), such as that collected by the Census Bureau. Sometimes called Prevalence Studies.
struct CrossSectional;

/// A convenience store.
struct ConvenienceStore;

/// Component (sub-)structure(s) that comprise this anatomical structure.
struct subStructure;

/// The act of forming one's opinion, reaction or sentiment.
struct AssessAction;

/// A sub property of instrument. The language used on this action.
struct language;

/// The observationDate of an [[Observation]].
struct observationDate;

/// A bar or pub.
struct BarOrPub;

/// Specifies browser requirements in human-readable text. For example, 'requires HTML5 support'.
struct browserRequirements;

/// A competitor in a sports event.
struct competitor;

/// A PerformanceRole is a Role that some entity places with regard to a theatrical performance, e.g. in a Movie, TVSeries etc.
struct PerformanceRole;

/// Changes in the normal mechanical, physical, and biochemical functions that are associated with this activity or condition.
struct pathophysiology;

/// A description of costs to the patient under a given network or formulary.
struct HealthPlanCostSharingSpecification;

/// A shopping center or mall.
struct ShoppingCenter;

/// Indicates whether this image is representative of the content of the page.
struct representativeOfPage;

/// Of a [[Person]], and less typically of an [[Organization]], to indicate a topic that is known about - suggesting possible expertise but not implying it. We do not distinguish skill levels here, or relate this to educational content, events, objectives or [[JobPosting]] descriptions.
struct knowsAbout;

/// Terminated.
struct Terminated;

/// Nonprofit501c19: Non-profit type referring to Post or Organization of Past or Present Members of the Armed Forces.
struct Nonprofit501c19;

/// This stands for any day that is a public holiday; it is a placeholder for all official public holidays in some particular location. While not technically a "day of the week", it can be used with [[OpeningHoursSpecification]]. In the context of an opening hours specification it can be used to indicate opening hours on public holidays, overriding general opening hours for the day of the week on which a public holiday occurs.
struct PublicHolidays;

/// The rating given in this review. Note that reviews can themselves be rated. The ```reviewRating``` applies to rating given by the review. The [[aggregateRating]] property applies to the review itself, as a creative work.
struct reviewRating;

/// A work performed in some event, for example a play performed in a TheaterEvent.
struct workPerformed;

/// Web page type: Media gallery page. A mixed-media page that can contains media such as images, videos, and other multimedia.
struct MediaGallery;

/// A sub property of participant. The participant who is at the sending end of the action.
struct sender;

/// A human-readable summary of specific accessibility features or deficiencies, consistent with the other accessibility metadata but expressing subtleties such as "short descriptions are present but long descriptions will be needed for non-visual users" or "short descriptions are present and no long descriptions are needed."
struct accessibilitySummary;

/// People or organizations that endorse the plan.
struct endorsers;

/// The typical delay between the receipt of the order and the goods either leaving the warehouse or being prepared for pickup, in case the delivery method is on site pickup.
struct deliveryLeadTime;

/// A specific branch of medical science that is concerned with the diagnosis and treatment of diseases, debilities and provision of care to the aged.
struct Geriatric;

/// The subject matter of the content.
struct about;

/// A [[CategoryCodeSet]] that contains this category code.
struct inCodeSet;

/// A unique instance of a radio BroadcastService on a CableOrSatelliteService lineup.
struct RadioChannel;

/// The International Standard Recording Code for the recording.
struct isrcCode;

/// The act of consuming dynamic/moving visual content.
struct WatchAction;

/// One of the domain specialities to which this web page's content applies.
struct specialty;

/// The place of attachment of a muscle, or what the muscle moves.
struct insertion;

/// A commonly used identifier for the characteristic represented by the property, e.g. a manufacturer or a standard code for a property. propertyID can be
/// (1) a prefixed string, mainly meant to be used with standards for product properties; (2) a site-specific, non-prefixed string (e.g. the primary key of the property or the vendor-specific id of the property), or (3)
/// a URL indicating the type of the property, either pointing to an external vocabulary, or a Web resource that describes the property (e.g. a glossary entry).
/// Standards bodies should promote a standard prefix for the identifiers of properties from their standards.
struct propertyID;

/// The maximum number of students who may be enrolled in the program.
struct maximumEnrollment;

/// VinylFormat.
struct VinylFormat;

/// Pathogenic fungus.
struct Fungus;

/// Any precaution, guidance, contraindication, etc. related to consumption of alcohol while taking this drug.
struct alcoholWarning;

/// indicates (possibly multiple) shipping destinations. These can be defined in several ways e.g. postalCode ranges.
struct shippingDestination;

/// (Eventually to be defined as) a supertype of GeoShape designed to accommodate definitions from Geo-Spatial best practices.
struct GeospatialGeometry;

/// The number of pages in the book.
struct numberOfPages;

/// Text representing an XPath (typically but not necessarily version 1.0).
struct XPathType;

/// The date of the first registration of the vehicle with the respective public authorities.
struct dateVehicleFirstRegistered;

/// A circle is the circular region of a specified radius centered at a specified latitude and longitude. A circle is expressed as a pair followed by a radius in meters.
struct circle;

/// A police station.
struct PoliceStation;

/// A nurse-like health profession that deals with pregnancy, childbirth, and the postpartum period (including care of the newborn), besides sexual and reproductive health of women throughout their lives.
struct Midwifery;

/// Library file name e.g., mscorlib.dll, system.web.dll.
struct executableLibraryName;

/// The minimum payment is the lowest amount of money that one is required to pay on a credit card statement each month.
struct monthlyMinimumRepaymentAmount;

/// Physical activity that is engaged to help maintain posture and balance.
struct Balance;

/// Indicates that the item is in stock.
struct InStock;

/// A music album.
struct album;

/// Indicates the [NATO stock number](https://en.wikipedia.org/wiki/NATO_Stock_Number) (nsn) of a [[Product]]. 
struct nsn;

/// Podiatry is the care of the human foot, especially the diagnosis and treatment of foot disorders.
struct Podiatric;

/// An action performed by a direct agent and indirect participants upon a direct object. Optionally happens at a location with the help of an inanimate instrument. The execution of the action may produce a result. Specific action sub-type documentation specifies the exact expectation of each argument/role.\n\nSee also [blog post](http://blog.schema.org/2014/04/announcing-schemaorg-actions.html) and [Actions overview document](https://schema.org/docs/actions.html).
struct Action;

/// Date of first broadcast/publication.
struct datePublished;

/// An educational or occupational credential. A diploma, academic degree, certification, qualification, badge, etc., that may be awarded to a person or other entity that meets the requirements defined by the credentialer.
struct EducationalOccupationalCredential;

/// The name displayed in the channel guide. For many US affiliates, it is the network name.
struct broadcastDisplayName;

/// A dosing schedule for the drug for a given population, either observed, recommended, or maximum dose based on the type used.
struct doseSchedule;

/// The quantity of the goods included in the offer.
struct amountOfThisGood;

/// A type of permission which can be granted for accessing a digital document.
struct DigitalDocumentPermissionType;

/// The number of items in the [[Collection]].
struct collectionSize;

/// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
struct numberOfCredits;

/// Attraction located at destination.
struct includesAttraction;

/// A treatment of people with physical, emotional, or social problems, using purposeful activity to help them overcome or learn to deal with their problems.
struct OccupationalTherapy;

/// Indicates whether the vehicle has been used for special purposes, like commercial rental, driving school, or as a taxi. The legislation in many countries requires this information to be revealed when offering a car for sale.
struct vehicleSpecialUsage;

/// Status of a game server.
struct serverStatus;

/// A table on a Web page.
struct Table;

/// Indicates that the item is new.
struct NewCondition;

/// A department store.
struct DepartmentStore;

/// A web page. Every web page is implicitly assumed to be declared to be of type WebPage, so the various properties about that webpage, such as <code>breadcrumb</code> may be used. We recommend explicit declaration if these properties are specified, but if they are found outside of an itemscope, they will be assumed to be about the page.
struct WebPage;

/// The anatomical location at which two or more bones make contact.
struct Joint;

/// An image containing a diagram that illustrates the structure and/or its component substructures and/or connections with other structures.
struct diagram;

/// A motorcycle dealer.
struct MotorcycleDealer;

/// A tool used (but not consumed) when performing instructions for how to achieve a result.
struct HowToTool;

/// The individual who adds color to inked drawings.
struct colorist;

/// Length of time to engage in the activity.
struct activityDuration;

/// The date on which the CreativeWork was created or the item was added to a DataFeed.
struct dateCreated;

/// A medical service available from this provider.
struct availableService;

/// A public structure, such as a town hall or concert hall.
struct CivicStructure;

/// A condition or factor that indicates use of a medical therapy, including signs, symptoms, risk factors, anatomical states, etc.
struct MedicalIndication;

/// Financial services business.
struct FinancialService;

/// (editorial work in progress, this definition is incomplete and unreviewed)
///     A [[MediaReview]] is a more specialized form of Review dedicated to the evaluation of media content online, typically in the context of fact-checking and misinformation.
///     For more general reviews of media in the broader sense, use [[UserReview]], [[CriticReview]] or other [[Review]] types.
struct MediaReview;

/// Indicates a [[HyperTocEntry]] in a [[HyperToc]].
struct tocEntry;

/// The act of providing goods, services, or money without compensation, often for philanthropic reasons.
struct DonateAction;

/// A work that is a translation of the content of this work. e.g. 西遊記 has an English workTranslation “Journey to the West”,a German workTranslation “Monkeys Pilgerfahrt” and a Vietnamese  translation Tây du ký bình khảo.
struct workTranslation;

/// The address for accessing the service by mail.
struct servicePostalAddress;

/// The length of time it takes to prepare the items to be used in instructions or a direction, in [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601).
struct prepTime;

/// A distillery.
struct Distillery;

/// The identifier for the [[Course]] used by the course [[provider]] (e.g. CS101 or 6.001).
struct courseCode;

/// A lodging business, such as a motel, hotel, or inn.
struct LodgingBusiness;

/// This is the [[Action]] of navigating to a specific [[startOffset]] timestamp within a [[VideoObject]], typically represented with a URL template structure.
struct SeekToAction;

/// CassetteFormat.
struct CassetteFormat;

/// A set of characteristics describing parents, who can be interested in viewing some content.
struct ParentAudience;

/// A News/Media organization such as a newspaper or TV station.
struct NewsMediaOrganization;

/// A URL to a map of the place.
struct hasMap;

/// A person that acts in a coaching role for a sports team.
struct coach;

/// Any physical manifestation of a person's medical condition discoverable by objective diagnostic tests or physical examination.
struct MedicalSign;

/// A placebo-controlled trial design.
struct PlaceboControlledTrial;

/// The type of engine or engines powering the vehicle.
struct engineType;

/// Related topics may be treated by a Topic.
struct MayTreatHealthAspect;

/// Specific qualifications required for this role or Occupation.
struct qualifications;

/// The event has been postponed and no new date has been set. The event's previousStartDate should be set.
struct EventPostponed;

/// A trial design in which the researcher knows the full details of the treatment, and so does the patient.
struct OpenTrial;

/// The person who wrote the words.
struct lyricist;

/// The CableOrSatelliteService offering the channel.
struct inBroadcastLineup;

/// The act of expressing a preference from a set of options or a large or unbounded set of choices/options.
struct ChooseAction;

/// A description of the procedure involved in setting up, using, and/or installing the device.
struct procedure;

/// The act of registering to an organization/service without the guarantee to receive it.\n\nRelated actions:\n\n* [[RegisterAction]]: Unlike RegisterAction, ApplyAction has no guarantees that the application will be accepted.
struct ApplyAction;

/// The price asked for a given offer by the respective organization or person.
struct UnitPriceSpecification;

/// The current status of the order item.
struct orderItemStatus;

/// The identifier for the account the payment will be applied to.
struct accountId;

/// Properties that take Distances as values are of the form '&lt;Number&gt; &lt;Length unit of measure&gt;'. E.g., '7 ft'.
struct Distance;

/// The date and time of giving up ownership on the product.
struct ownedThrough;

/// An ItemList ordered with lower values listed first.
struct ItemListOrderAscending;

/// An active ingredient, typically chemical compounds and/or biologic substances.
struct activeIngredient;

/// Any condition of the human body that affects the normal functioning of a person, whether physically or mentally. Includes diseases, injuries, disabilities, disorders, syndromes, etc.
struct MedicalCondition;

/// The branches that delineate from the nerve bundle. Not to be confused with [[branchOf]].
struct branch;

/// Indicates a correction to a [[CreativeWork]], either via a [[CorrectionComment]], textually or in another document.
struct correction;

/// Content about how to screen or further filter a topic.
struct ScreeningHealthAspect;

/// A risk factor is anything that increases a person's likelihood of developing or contracting a disease, medical condition, or complication.
struct MedicalRiskFactor;

/// A person who reads (performs) the audiobook.
struct readBy;

/// A food service, like breakfast, lunch, or dinner.
struct FoodService;

/// A patient-reported or observed dosing schedule for a drug or supplement.
struct ReportedDoseSchedule;

/// The number of grams of fiber.
struct fiberContent;

/// The start time of the clip expressed as the number of seconds from the beginning of the work.
struct startOffset;

/// Appearance assessment with clinical examination.
struct Appearance;

/// Self care actions or measures that can be taken to sooth, health or avoid a topic. This may be carried at home and can be carried/managed by the person itself.
struct SelfCareHealthAspect;

/// A diagnostic test or procedure offered by this lab.
struct availableTest;

/// Any potential safety concern associated with the supplement. May include interactions with other drugs and foods, pregnancy, breastfeeding, known adverse reactions, and documented efficacy of the supplement.
struct safetyConsideration;

/// Indicates that the publisher gives some special status to the publication of the document. ("The Queens Printer" version of a UK Act of Parliament, or the PDF version of a Directive published by the EU Office of Publications). Something "Authoritative" is considered to be also [[OfficialLegalValue]]".
struct AuthoritativeLegalValue;

/// A recipe. For dietary restrictions covered by the recipe, a few common restrictions are enumerated via [[suitableForDiet]]. The [[keywords]] property can also be used to add more detail.
struct Recipe;

/// A sea (for example, the Caspian sea).
struct SeaBodyOfWater;

/// The number of offers for the product.
struct offerCount;

/// Specifies the CreativeWork associated with the UserComment.
struct discusses;

/// The degree of mobility the joint allows.
struct functionalClass;

/// A short text indicating the configuration of the vehicle, e.g. '5dr hatchback ST 2.5 MT 225 hp' or 'limited edition'.
struct vehicleConfiguration;

/// Indicates that a document has no particular or special standing (e.g. a republication of a law by a private publisher).
struct UnofficialLegalValue;

/// The number of employees in an organization e.g. business.
struct numberOfEmployees;

/// Nonprofit501c17: Non-profit type referring to Supplemental Unemployment Benefit Trusts.
struct Nonprofit501c17;

/// Represents EU Energy Efficiency Class A++ as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryA2Plus;

/// Further documentation describing the Web API in more detail.
struct documentation;

/// A notary.
struct Notary;

/// StudioAlbum.
struct StudioAlbum;

/// A type of Bank Account with a main purpose of depositing funds to gain interest or other benefits.
struct DepositAccount;

/// Status of a game server.
struct GameServerStatus;

/// An auto parts store.
struct AutoPartsStore;

/// The act of physically/electronically taking delivery of an object that has been transferred from an origin to a destination. Reciprocal of SendAction.\n\nRelated actions:\n\n* [[SendAction]]: The reciprocal of ReceiveAction.\n* [[TakeAction]]: Unlike TakeAction, ReceiveAction does not imply that the ownership has been transfered (e.g. I can receive a package, but it does not mean the package is now mine).
struct ReceiveAction;

/// The act of authoring written creative content.
struct WriteAction;

/// Library file name e.g., mscorlib.dll, system.web.dll.
struct assembly;

/// The geographic area associated with the audience.
struct geographicArea;

/// Indicates that the item has sold out.
struct SoldOut;

/// A complex mathematical calculation requiring an online calculator, used to assess prognosis. Note: use the url property of Thing to record any URLs for online calculators.
struct MedicalRiskCalculator;

/// Computer programming source code. Example: Full (compile ready) solutions, code snippet samples, scripts, templates.
struct SoftwareSourceCode;

/// A Service to transfer funds from a person or organization to a beneficiary person or organization.
struct PaymentService;

/// Nonprofit501c10: Non-profit type referring to Domestic Fraternal Societies and Associations.
struct Nonprofit501c10;

/// A landform or physical feature.  Landform elements include mountains, plains, lakes, rivers, seascape and oceanic waterbody interface features such as bays, peninsulas, seas and so forth, including sub-aqueous terrain features such as submersed mountain ranges, volcanoes, and the great ocean basins.
struct Landform;

/// Represents EU Energy Efficiency Class D as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryD;

/// The place where the Organization was founded.
struct foundingLocation;

/// A scholarly article in the medical domain.
struct MedicalScholarlyArticle;

/// The shipping rate is the cost of shipping to the specified destination. Typically, the maxValue and currency values (of the [[MonetaryAmount]]) are most appropriate.
struct shippingRate;

/// The estimated salary earned while in the program.
struct trainingSalary;

/// A shop that will buy, or lend money against the security of, personal possessions.
struct PawnShop;

/// Another legislation that this legislation changes. This encompasses the notions of amendment, replacement, correction, repeal, or other types of change. This may be a direct change (textual or non-textual amendment) or a consequential or indirect change. The property is to be used to express the existence of a change relationship between two acts rather than the existence of a consolidated version of the text that shows the result of the change. For consolidation relationships, use the <a href="/legislationConsolidates">legislationConsolidates</a> property.
struct legislationChanges;

/// The date on which the CreativeWork was most recently modified or when the item's entry was modified within a DataFeed.
struct dateModified;

/// The position of an item in a series or sequence of items.
struct position;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserPageVisits;

/// The value of the dose, e.g. 500.
struct doseValue;

/// The scientific study and treatment of defects, disorders, and malfunctions of speech and voice, as stuttering, lisping, or lalling, and of language disturbances, as aphasia or delayed language acquisition.
struct SpeechPathology;

/// Enumerated status values for Order.
struct OrderStatus;

/// A specific branch of medical science that pertains to treating diseases, injuries and deformities by manual and instrumental means.
struct Surgical;

/// A reservation for air travel.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
struct FlightReservation;

/// Information about questions that may be asked, when to see a professional, measures before seeing a doctor or content about the first consultation.
struct SeeDoctorHealthAspect;

/// The act of reaching a draw in a competitive activity.
struct TieAction;

/// Data type: Text.
struct Text;

/// An organization identifier that uniquely identifies a legal entity as defined in ISO 17442.
struct leiCode;

/// Used in conjunction with eventStatus for rescheduled or cancelled events. This property contains the previously scheduled start date. For rescheduled events, the startDate property should be used for the newly scheduled start date. In the (rare) case of an event that has been postponed and rescheduled multiple times, this field may be repeated.
struct previousStartDate;

/// An art gallery.
struct ArtGallery;

/// A motorcycle repair shop.
struct MotorcycleRepair;

/// A sports location, such as a playing field.
struct SportsActivityLocation;

/// The boolean value true.
struct True;

/// Event type: Music event.
struct MusicEvent;

/// A medical specialty of the provider.
struct medicalSpecialty;

/// A designation by the US FDA signifying that there is positive evidence of human fetal risk based on adverse reaction data from investigational or marketing experience or studies in humans, but potential benefits may warrant use of the drug in pregnant women despite potential risks.
struct FDAcategoryD;

/// The duration for which the given offer is valid.
struct eligibleDuration;

/// A modifiable or non-modifiable factor that increases the risk of a patient contracting this condition, e.g. age,  coexisting condition.
struct riskFactor;

/// Anatomical systems or structures that relate to the superficial anatomy.
struct relatedAnatomy;

/// A reservation for lodging at a hotel, motel, inn, etc.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations.
struct LodgingReservation;

/// The number of milligrams of sodium.
struct sodiumContent;

/// The row location of the reserved seat (e.g., B).
struct seatRow;

/// LiveAlbum.
struct LiveAlbum;

/// Review of the item.
struct reviews;

/// A FloorPlan is an explicit representation of a collection of similar accommodations, allowing the provision of common information (room counts, sizes, layout diagrams) and offers for rental or sale. In typical use, some [[ApartmentComplex]] has an [[accommodationFloorPlan]] which is a [[FloorPlan]].  A FloorPlan is always in the context of a particular place, either a larger [[ApartmentComplex]] or a single [[Apartment]]. The visual/spatial aspects of a floor plan (i.e. room layout, [see wikipedia](https://en.wikipedia.org/wiki/Floor_plan)) can be indicated using [[image]]. 
struct FloorPlan;

/// The available volume for cargo or luggage. For automobiles, this is usually the trunk volume.\n\nTypical unit code(s): LTR for liters, FTQ for cubic foot/feet\n\nNote: You can use [[minValue]] and [[maxValue]] to indicate ranges.
struct cargoVolume;

/// A trial design in which the researcher knows which treatment the patient was randomly assigned to but the patient does not.
struct SingleBlindedTrial;

/// Awards won by or for this item.
struct awards;

/// The event has been rescheduled. The event's previousStartDate should be set to the old date and the startDate should be set to the event's new date. (If the event has been rescheduled multiple times, the previousStartDate property may be repeated).
struct EventRescheduled;

/// A screening of a movie or other video.
struct ScreeningEvent;

/// Play mode: CoOp. Co-operative games, where you play on the same team with friends.
struct CoOp;

/// A sub property of location. The course where this action was taken.
struct exerciseCourse;

/// When a taxi will pickup a passenger or a rental car can be picked up.
struct pickupTime;

/// Indicates the status of drug prescription eg. local catalogs classifications or whether the drug is available by prescription or over-the-counter, etc.
struct prescriptionStatus;

/// The target group associated with a given audience (e.g. veterans, car owners, musicians, etc.).
struct audienceType;

/// A set of Category Code values.
struct CategoryCodeSet;

/// An organization that this person is affiliated with. For example, a school/university, a club, or a team.
struct affiliation;

/// A medical therapy related to this anatomy.
struct relatedTherapy;

/// Unlike cross-sectional studies, longitudinal studies track the same people, and therefore the differences observed in those people are less likely to be the result of cultural differences across generations. Longitudinal studies are also used in medicine to uncover predictors of certain diseases.
struct Longitudinal;

/// The person or organization the reservation or ticket is for.
struct underName;

/// A sub property of participant. The loser of the action.
struct loser;

/// An ItemList ordered with no explicit order.
struct ItemListUnordered;

/// A guideline contraindication that designates a process as harmful and where quality of the data supporting the contraindication is sound.
struct MedicalGuidelineContraindication;

/// The modulation (e.g. FM, AM, etc) used by a particular broadcast service
struct broadcastSignalModulation;

/// A means for accessing a service, e.g. a government office location, web site, or phone number.
struct ServiceChannel;

/// A medical study or trial related to this entity.
struct study;

/// The artwork on the outer surface of a CreativeWork.
struct CoverArt;

/// Event type: Food event.
struct FoodEvent;

/// The number of doors.\n\nTypical unit code(s): C62
struct numberOfDoors;

/// The task that a player-controlled character, or group of characters may complete in order to gain a reward.
struct quest;

/// The author of this content or rating. Please note that author is special in that HTML 5 provides a special mechanism for indicating authorship via the rel tag. That is equivalent to this and may be used interchangeably.
struct author;

/// A motorized bicycle is a bicycle with an attached motor used to power the vehicle, or to assist with pedaling.
struct MotorizedBicycle;

/// An [[Article]] whose content is primarily [[satirical]](https://en.wikipedia.org/wiki/Satire) in nature, i.e. unlikely to be literally true. A satirical article is sometimes but not necessarily also a [[NewsArticle]]. [[ScholarlyArticle]]s are also sometimes satirized.
struct SatiricalArticle;

/// A Property value specification.
struct PropertyValueSpecification;

/// A hotel is an establishment that provides lodging paid on a short-term basis (Source: Wikipedia, the free encyclopedia, see http://en.wikipedia.org/wiki/Hotel).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct Hotel;

/// The passenger's sequence number as assigned by the airline.
struct passengerSequenceNumber;

/// A bank or bank’s branch, financial institution or international financial institution operating the beneficiary’s bank account or releasing funds for the beneficiary
struct beneficiaryBank;

/// Nonprofit501c8: Non-profit type referring to Fraternal Beneficiary Societies and Associations.
struct Nonprofit501c8;

/// The time when the live blog will stop covering the Event. Note that coverage may continue after the Event concludes.
struct coverageEndTime;

/// An award won by or for this item.
struct award;

/// Headline of the article.
struct headline;

/// The invitee will attend.
struct RsvpResponseYes;

/// Stages that can be observed from a topic.
struct StagesHealthAspect;

/// Audiences defined by a person's minimum age.
struct requiredMinAge;

/// The area to which the artery supplies blood.
struct supplyTo;

/// A structured value representing repayment.
struct RepaymentSpecification;

/// The Order(s) related to this Invoice. One or more Orders may be combined into a single Invoice.
struct referencesOrder;

/// A StatisticalPopulation is a set of instances of a certain given type that satisfy some set of constraints. The property [[populationType]] is used to specify the type. Any property that can be used on instances of that type can appear on the statistical population. For example, a [[StatisticalPopulation]] representing all [[Person]]s with a [[homeLocation]] of East Podunk California, would be described by applying the appropriate [[homeLocation]] and [[populationType]] properties to a [[StatisticalPopulation]] item that stands for that set of people.
/// The properties [[numConstraints]] and [[constrainingProperties]] are used to specify which of the populations properties are used to specify the population. Note that the sense of "population" used here is the general sense of a statistical
/// population, and does not imply that the population consists of people. For example, a [[populationType]] of [[Event]] or [[NewsArticle]] could be used. See also [[Observation]], and the [data and datasets](/docs/data-and-datasets.html) overview for more details.
///   
struct StatisticalPopulation;

/// Recruiting participants.
struct Recruiting;

/// For itemListElement values, you can use simple strings (e.g. "Peter", "Paul", "Mary"), existing entities, or use ListItem.\n\nText values are best if the elements in the list are plain strings. Existing entities are best for a simple, unordered list of existing things in your data. ListItem is used with ordered lists when you want to provide additional context about the element in that list or when the same item might be in different places in different lists.\n\nNote: The order of elements in your mark-up is not sufficient for indicating the order or elements.  Use ListItem with a 'position' property in such cases.
struct itemListElement;

/// Any discount applied (to an Order).
struct discount;

/// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
struct recipeYield;

/// The Vehicle Identification Number (VIN) is a unique serial number used by the automotive industry to identify individual motor vehicles.
struct vehicleIdentificationNumber;

/// A component test of the panel.
struct subTest;

/// The significance associated with the superficial anatomy; as an example, how characteristics of the superficial anatomy can suggest underlying medical conditions or courses of treatment.
struct significance;

/// A datasheet or vendor specification of a product (in the sense of a prototypical description).
struct ProductModel;

/// A sibling of the person.
struct sibling;

/// A media object that encodes this CreativeWork. This property is a synonym for encoding.
struct associatedMedia;

/// Indicates the usage of the car as a taxi.
struct TaxiVehicleUsage;

/// The main performer or performers of the event&#x2014;for example, a presenter, musician, or actor.
struct performers;

/// The act of stopping or deactivating a device or application (e.g. stopping a timer or turning off a flashlight).
struct DeactivateAction;

/// A diet focused on reduced fat and cholesterol intake.
struct LowFatDiet;

/// NonprofitANBI: Non-profit type referring to a Public Benefit Organization (NL).
struct NonprofitANBI;

/// ICAO identifier for an airport.
struct icaoCode;

/// The class of drug this belongs to (e.g., statins).
struct drugClass;

/// True if this item's name is a proprietary/brand name (vs. generic name).
struct isProprietary;

/// An Insurance agency.
struct InsuranceAgency;

/// Indicates a range of postalcodes, usually defined as the set of valid codes between [[postalCodeBegin]] and [[postalCodeEnd]], inclusively.
struct PostalCodeRangeSpecification;

/// The date/time at which the message was sent.
struct dateSent;

/// Indicates the aspect or aspects specifically addressed in some [[HealthTopicContent]]. For example, that the content is an overview, or that it talks about treatment, self-care, treatments or their side-effects.
struct hasHealthAspect;

/// A sub property of participant. The owner of the real estate property.
struct landlord;

/// Estimated processing time for the service using this channel.
struct processingTime;

/// A motorcycle or motorbike is a single-track, two-wheeled motor vehicle.
struct Motorcycle;

/// The URL that goes directly to the summary of benefits and coverage for the specific standard plan or plan variation.
struct benefitsSummaryUrl;

/// Represents EnergyStar certification
struct EnergyStarCertified;

/// A FundingAgency is an organization that implements one or more [[FundingScheme]]s and manages
///     the granting process (via [[Grant]]s, typically [[MonetaryGrant]]s).
///     A funding agency is not always required for grant funding, e.g. philanthropic giving, corporate sponsorship etc.
///     
/// Examples of funding agencies include ERC, REA, NIH, Bill and Melinda Gates Foundation...
///     
struct FundingAgency;

/// Number of partial bathrooms - The total number of half and ¼ bathrooms in an [[Accommodation]]. This corresponds to the [BathroomsPartial field in RESO](https://ddwiki.reso.org/display/DDW17/BathroomsPartial+Field). 
struct numberOfPartialBathrooms;

/// Any medical imaging modality typically used for diagnostic purposes. Enumerated type.
struct MedicalImagingTechnique;

/// The speed range of the vehicle. If the vehicle is powered by an engine, the upper limit of the speed range (indicated by [[maxValue]] should be the maximum speed achievable under regular conditions.\n\nTypical unit code(s): KMH for km/h, HM for mile per hour (0.447 04 m/s), KNT for knot\n\n*Note 1: Use [[minValue]] and [[maxValue]] to indicate the range. Typically, the minimal value is zero.\n* Note 2: There are many different ways of measuring the speed range. You can link to information about how the given value has been determined using the [[valueReference]] property.
struct speed;

/// Indicates that parts of the legislation are in force, and parts are not.
struct PartiallyInForce;

/// The color or color combination of the interior of the vehicle.
struct vehicleInteriorColor;

/// The act of planning the execution of an event/task/action/reservation/plan to a future date.
struct PlanAction;

/// Specifies the least energy efficient class on the regulated EU energy consumption scale for the product category a product belongs to. For example, energy consumption for televisions placed on the market after January 1, 2020 is scaled from D to A+++.
struct energyEfficiencyScaleMin;

/// A medical organization (physical or not), such as hospital, institution or clinic.
struct MedicalOrganization;

/// Indicates the main image on the page.
struct primaryImageOfPage;

/// Specifies for how long this price (or price component) will be billed. Can be used, for example, to model the contractual duration of a subscription or payment plan. Type can be either a Duration or a Number (in which case the unit of measurement, for example month, is specified by the unitCode property)
struct billingDuration;

/// A bike store.
struct BikeStore;

/// A season in a media series.
struct seasons;

/// The day of the week between Tuesday and Thursday.
struct Wednesday;

/// The object that helped the agent perform the action. e.g. John wrote a book with *a pen*.
struct instrument;

/// Links to tips, tactics, etc.
struct gameTip;

/// numc19died - DEATHS: Patients with suspected or confirmed COVID-19 who died in the hospital, ED, or any overflow location.
struct cvdNumC19Died;

/// A private parcel service as the delivery mode available for a certain offer.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#DHL\n* http://purl.org/goodrelations/v1#FederalExpress\n* http://purl.org/goodrelations/v1#UPS
///       
struct ParcelService;

/// A SpeakableSpecification indicates (typically via [[xpath]] or [[cssSelector]]) sections of a document that are highlighted as particularly [[speakable]]. Instances of this type are expected to be used primarily as values of the [[speakable]] property.
struct SpeakableSpecification;

/// A math solver which is capable of solving a subset of mathematical problems.
struct MathSolver;

/// Method used for delivery or shipping.
struct hasDeliveryMethod;

/// A data catalog which contains this dataset.
struct includedInDataCatalog;

/// Researchers.
struct Researcher;

/// A Research project.
struct ResearchProject;

/// A country.
struct Country;

/// Description of the absorption and elimination of drugs, including their concentration (pharmacokinetics, pK) and biological effects (pharmacodynamics, pD).
struct clinicalPharmacology;

/// USNonprofitType: Non-profit organization type originating from the United States.
struct USNonprofitType;

/// Web page type: Video gallery page.
struct VideoGallery;

/// A diet focused on reduced calorie intake.
struct LowCalorieDiet;

/// The offer(s) -- e.g., product, quantity and price combinations -- included in the order.
struct acceptedOffer;

/// MerchantReturnFiniteReturnWindow: there is a finite window for product returns.
struct MerchantReturnFiniteReturnWindow;

/// Scheduling future actions, events, or tasks.\n\nRelated actions:\n\n* [[ReserveAction]]: Unlike ReserveAction, ScheduleAction allocates future actions (e.g. an event, a task, etc) towards a time slot / spatial allocation.
struct ScheduleAction;

/// Recommended intake of this supplement for a given population as defined by a specific recommending authority.
struct maximumIntake;

/// Maximal age recommended for viewing content.
struct suggestedMaxAge;

/// A specific strength in which a medical drug is available in a specific country.
struct DrugStrength;

/// The stage represented as a number, e.g. 3.
struct stageAsNumber;

/// The cuisine of the recipe (for example, French or Ethiopian).
struct recipeCuisine;

/// The payment method(s) to which the payment charge specification applies.
struct appliesToPaymentMethod;

/// The ISBN of the book.
struct isbn;

/// A book, document, or piece of music written by hand rather than typed or printed.
struct Manuscript;

/// Medical clinicians, including practicing physicians and other medical professionals involved in clinical practice.
struct Clinician;

/// The price range of the business, for example ```$$$```.
struct priceRange;

/// A (typically single) geographic location associated with the job position.
struct jobLocation;

/// The words in the song.
struct lyrics;

/// Professional service: Attorney. \n\nThis type is deprecated - [[LegalService]] is more inclusive and less ambiguous.
struct Attorney;

/// The act of installing an application.
struct InstallAction;

/// OrderStatus representing that an order has been returned.
struct OrderReturned;

/// nonprofit Status indicates the legal status of a non-profit organization in its primary place of business.
struct nonprofitStatus;

/// Indicates a page with news updates and guidelines. This could often be (but is not required to be) the main page containing [[SpecialAnnouncement]] markup on a site.
struct newsUpdatesAndGuidelines;

/// A medical laboratory that offers on-site or off-site diagnostic services.
struct DiagnosticLab;

/// Relates a property to a class that is (one of) the type(s) the property is expected to be used on.
struct domainIncludes;

/// Typical preparation that a patient must undergo before having the procedure performed.
struct preparation;

/// DigitalAudioTapeFormat.
struct DigitalAudioTapeFormat;

/// A placeholder for multiple similar products of the same kind.
struct SomeProducts;

/// OfflineEventAttendanceMode - an event that is primarily conducted offline. 
struct OfflineEventAttendanceMode;

/// Information about the engine or engines of the vehicle.
struct vehicleEngine;

/// The act of being defeated in a competitive activity.
struct LoseAction;

/// Specifies the Person who edited the CreativeWork.
struct editor;

/// A trial design in which neither the researcher nor the patient knows the details of the treatment the patient was randomly assigned to.
struct DoubleBlindedTrial;

/// Server that provides game interaction in a multiplayer game.
struct GameServer;

/// The currency of the discount.\n\nUse standard formats: [ISO 4217 currency format](http://en.wikipedia.org/wiki/ISO_4217) e.g. "USD"; [Ticker symbol](https://en.wikipedia.org/wiki/List_of_cryptocurrencies) for cryptocurrencies e.g. "BTC"; well known names for [Local Exchange Tradings Systems](https://en.wikipedia.org/wiki/Local_exchange_trading_system) (LETS) and other currency types e.g. "Ithaca HOUR".
struct discountCurrency;

/// Organization: A business corporation.
struct Corporation;

/// Text representing a CSS selector.
struct CssSelectorType;

/// The group the release is credited to if different than the byArtist. For example, Red and Blue is credited to "Stefani Germanotta Band", but by Lady Gaga.
struct creditedTo;

/// The expected departure time.
struct departureTime;

/// OriginalShippingFees ...
struct OriginalShippingFees;

/// A hair salon.
struct HairSalon;

/// Nonprofit501d: Non-profit type referring to Religious and Apostolic Associations.
struct Nonprofit501d;

/// A thumbnail image relevant to the Thing.
struct thumbnailUrl;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserTweets;

/// Nonprofit501k: Non-profit type referring to Child Care Organizations.
struct Nonprofit501k;

/// The caption for this object. For downloadable machine formats (closed caption, subtitles etc.) use MediaObject and indicate the [[encodingFormat]].
struct caption;

/// A medical study is an umbrella type covering all kinds of research studies relating to human medicine or health, including observational studies and interventional trials and registries, randomized, controlled or not. When the specific type of study is known, use one of the extensions of this type, such as MedicalTrial or MedicalObservationalStudy. Also, note that this type should be used to mark up data that describes the study itself; to tag an article that publishes the results of a study, use MedicalScholarlyArticle. Note: use the code property of MedicalEntity to store study IDs, e.g. clinicaltrials.gov ID.
struct MedicalStudy;

/// Organization: Sports team.
struct SportsTeam;

/// The Game type represents things which are games. These are typically rule-governed recreational activities, e.g. role-playing games in which players assume the role of characters in a fictional setting.
struct Game;

/// The number of grams of carbohydrates.
struct carbohydrateContent;

/// A sub property of object. The object that is being replaced.
struct replacee;

/// The default value of the input.  For properties that expect a literal, the default is a literal value, for properties that expect an object, it's an ID reference to one of the current values.
struct defaultValue;

/// For a [[NewsMediaOrganization]] or other news-related [[Organization]], a statement about public engagement activities (for news media, the newsroom’s), including involving the public - digitally or otherwise -- in coverage decisions, reporting and activities after publication.
struct actionableFeedbackPolicy;

/// An episode of a tv, radio or game media within a series or season.
struct episode;

/// The price for the delivery of an offer using a particular delivery method.
struct DeliveryChargeSpecification;

/// The act of giving money voluntarily to a beneficiary in recognition of services rendered.
struct TipAction;

/// Identifies input methods that are sufficient to fully control the described resource ([WebSchemas wiki lists possible values](http://www.w3.org/wiki/WebSchemas/Accessibility)).
struct accessibilityControl;

/// A type of physical examination of a patient performed by a physician. 
struct PhysicalExam;

/// A Category Code.
struct CategoryCode;

/// If this MediaObject is an AudioObject or VideoObject, the transcript of that object.
struct transcript;

/// An event venue.
struct EventVenue;

/// A web page element, like a table or an image.
struct WebPageElement;

/// A legislative building&#x2014;for example, the state capitol.
struct LegislativeBuilding;

/// The seat associated with the ticket.
struct ticketedSeat;

/// The time the object is scheduled to.
struct scheduledTime;

/// Shipper tracking number.
struct trackingNumber;

/// DJMixAlbum.
struct DJMixAlbum;

/// Represents EU Energy Efficiency Class A+++ as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryA3Plus;

/// Maximal age of the child.
struct childMaxAge;

/// Whether the provider is accepting new patients.
struct isAcceptingNewPatients;

/// Quantitative measure gauging the degree of force involved in the exercise, for example, heartbeats per minute. May include the velocity of the movement.
struct intensity;

/// A [[FAQPage]] is a [[WebPage]] presenting one or more "[Frequently asked questions](https://en.wikipedia.org/wiki/FAQ)" (see also [[QAPage]]).
struct FAQPage;

/// A muscle is an anatomical structure consisting of a contractile form of tissue that animals use to effect movement.
struct Muscle;

/// The current status of the order.
struct orderStatus;

/// The country. For example, USA. You can also provide the two-letter [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1).
struct addressCountry;

/// A tire shop.
struct TireShop;

/// Whether prescriptions can be delivered by mail.
struct offersPrescriptionByMail;

/// A sports club.
struct SportsClub;

/// A sibling of the person.
struct siblings;

/// Indicates the kind of product that this is a variant of. In the case of [[ProductModel]], this is a pointer (from a ProductModel) to a base product from which this product is a variant. It is safe to infer that the variant inherits all product features from the base model, unless defined locally. This is not transitive. In the case of a [[ProductGroup]], the group description also serves as a template, representing a set of Products that vary on explicitly defined, specific dimensions only (so it defines both a set of variants, as well as which values distinguish amongst those variants). When used with [[ProductGroup]], this property can apply to any [[Product]] included in the group.
struct isVariantOf;

/// A statistical distribution of values.
struct QuantitativeValueDistribution;

/// An organization that the person is an alumni of.
struct alumniOf;

/// Text that can be used to credit person(s) and/or organization(s) associated with a published Creative Work.
struct creditText;

/// The act of searching for an object.\n\nRelated actions:\n\n* [[FindAction]]: SearchAction generally leads to a FindAction, but not necessarily.
struct SearchAction;

/// An observational study design.
struct Observational;

/// Any recommendation made by a standard society (e.g. ACC/AHA) or consensus statement that denotes how to diagnose and treat a particular condition. Note: this type should be used to tag the actual guideline recommendation; if the guideline recommendation occurs in a larger scholarly article, use MedicalScholarlyArticle to tag the overall article, not this type. Note also: the organization making the recommendation should be captured in the recognizingAuthority base property of MedicalEntity.
struct MedicalGuideline;

/// An eventStatus of an event represents its status; particularly useful when an event is cancelled or rescheduled.
struct eventStatus;

/// The mailing address.
struct PostalAddress;

/// The text of the UserComment.
struct commentText;

/// A contact point&#x2014;for example, a Customer Complaints department.
struct ContactPoint;

/// Conditions that affect the availability of, or method(s) of access to, an item. Typically used for real world items such as an [[ArchiveComponent]] held by an [[ArchiveOrganization]]. This property is not suitable for use as a general Web access control mechanism. It is expressed only in natural language.\n\nFor example "Available by appointment from the Reading Room" or "Accessible only from logged-in accounts ". 
struct conditionsOfAccess;

/// This property is deprecated, alongside the UserInteraction types on which it depended.
struct interactionCount;

/// Position of the episode within an ordered group of episodes.
struct episodeNumber;

/// The date and time of obtaining the product.
struct ownedFrom;

/// Car repair business.
struct AutoRepair;

/// An Event that is part of this event. For example, a conference event includes many presentations, each of which is a subEvent of the conference.
struct subEvent;

/// The release date of a vehicle model (often used to differentiate versions of the same make and model).
struct vehicleModelDate;

/// One of the sections into which a book is divided. A chapter usually has a section number or a name.
struct Chapter;

/// Range of acceptable values for a typical patient, when applicable.
struct normalRange;

/// What type of code sample: full (compile ready) solution, code snippet, inline code, scripts, template.
struct codeSampleType;

/// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
struct seller;

/// The act of taking money from a buyer in exchange for goods or services rendered. An agent sells an object, product, or service to a buyer for a price. Reciprocal of BuyAction.
struct SellAction;

/// Content about how, when, frequency and dosage of a topic.
struct UsageOrScheduleHealthAspect;

/// A ShippingRateSettings represents re-usable pieces of shipping information. It is designed for publication on an URL that may be referenced via the [[shippingSettingsLink]] property of an [[OfferShippingDetails]]. Several occurrences can be published, distinguished and matched (i.e. identified/referenced) by their different values for [[shippingLabel]].
struct ShippingRateSettings;

/// The [[ReportageNewsArticle]] type is a subtype of [[NewsArticle]] representing
///  news articles which are the result of journalistic news reporting conventions.
/// 
/// In practice many news publishers produce a wide variety of article types, many of which might be considered a [[NewsArticle]] but not a [[ReportageNewsArticle]]. For example, opinion pieces, reviews, analysis, sponsored or satirical articles, or articles that combine several of these elements.
/// 
/// The [[ReportageNewsArticle]] type is based on a stricter ideal for "news" as a work of journalism, with articles based on factual information either observed or verified by the author, or reported and verified from knowledgeable sources.  This often includes perspectives from multiple viewpoints on a particular issue (distinguishing news reports from public relations or propaganda).  News reports in the [[ReportageNewsArticle]] sense de-emphasize the opinion of the author, with commentary and value judgements typically expressed elsewhere.
/// 
/// A [[ReportageNewsArticle]] which goes deeper into analysis can also be marked with an additional type of [[AnalysisNewsArticle]].
struct ReportageNewsArticle;

/// A sub property of recipient. The recipient copied on a message.
struct ccRecipient;

/// A strategy of regulating the intake of food to achieve or maintain a specific health-related goal.
struct Diet;

/// The start date and time of the item (in [ISO 8601 date format](http://en.wikipedia.org/wiki/ISO_8601)).
struct startDate;

/// The act of an agent communicating (service provider, social media, etc) their departure of a previously reserved service (e.g. flight check in) or place (e.g. hotel).\n\nRelated actions:\n\n* [[CheckInAction]]: The antonym of CheckOutAction.\n* [[DepartAction]]: Unlike DepartAction, CheckOutAction implies that the agent is informing/confirming the end of a previously reserved service.\n* [[CancelAction]]: Unlike CancelAction, CheckOutAction implies that the agent is informing/confirming the end of a previously reserved service.
struct CheckOutAction;

/// An agent tracks an object for updates.\n\nRelated actions:\n\n* [[FollowAction]]: Unlike FollowAction, TrackAction refers to the interest on the location of innanimates objects.\n* [[SubscribeAction]]: Unlike SubscribeAction, TrackAction refers to  the interest on the location of innanimate objects.
struct TrackAction;

/// A structured value providing information about the opening hours of a place or a certain service inside a place.\n\n
/// The place is __open__ if the [[opens]] property is specified, and __closed__ otherwise.\n\nIf the value for the [[closes]] property is less than the value for the [[opens]] property then the hour range is assumed to span over the next day.
///       
struct OpeningHoursSpecification;

/// A [[NewsArticle]] expressing an open call by a [[NewsMediaOrganization]] asking the public for input, insights, clarifications, anecdotes, documentation, etc., on an issue, for reporting purposes.
struct AskPublicNewsArticle;

/// Available by prescription only.
struct PrescriptionOnly;

/// The act of managing by changing/editing the state of the object.
struct UpdateAction;

/// Web page type: About page.
struct AboutPage;

/// Represents the installment pricing component of the total price for an offered product
struct Installment;

/// How the disease spreads, either as a route or vector, for example 'direct contact', 'Aedes aegypti', etc.
struct transmissionMethod;

/// A description of the qualification, award, certificate, diploma or other educational credential awarded as a consequence of successful completion of this course or program.
struct educationalCredentialAwarded;

/// For an [[Organization]] (often but not necessarily a [[NewsMediaOrganization]]), a report on staffing diversity issues. In a news context this might be for example ASNE or RTDNA (US) reports, or self-reported.
struct diversityStaffingReport;

/// The lowest value allowed in this rating system. If worstRating is omitted, 1 is assumed.
struct worstRating;

/// The act of transferring/moving (abstract or concrete) animate or inanimate objects from one place to another.
struct TransferAction;

/// A process of care relying upon counseling, dialogue and communication  aimed at improving a mental health condition without use of drugs.
struct PsychologicalTreatment;

/// Web page type: Search results page.
struct SearchResultsPage;

/// Keywords or tags used to describe this content. Multiple entries in a keywords list are typically delimited by commas.
struct keywords;

/// A review created by an end-user (e.g. consumer, purchaser, attendee etc.), in contrast with [[CriticReview]].
struct UserReview;

/// A file composed primarily of text.
struct TextDigitalDocument;

/// A blog post.
struct BlogPosting;

/// Event type: A social dance.
struct DanceEvent;

/// The time when a passenger can check into the flight online.
struct webCheckinTime;

/// DVDFormat.
struct DVDFormat;

/// X-ray computed tomography imaging.
struct CT;

/// A trip or journey. An itinerary of visits to one or more places.
struct Trip;

/// numicubedsocc - ICU BED OCCUPANCY: Total number of staffed inpatient ICU beds that are occupied.
struct cvdNumICUBedsOcc;

/// An infectious disease is a clinically evident human disease resulting from the presence of pathogenic microbial agents, like pathogenic viruses, pathogenic bacteria, fungi, protozoa, multicellular parasites, and prions. To be considered an infectious disease, such pathogens are known to be able to cause this disease.
struct InfectiousDisease;

/// An agent inspects, determines, investigates, inquires, or examines an object's accuracy, quality, condition, or state.
struct CheckAction;

/// A computer store.
struct ComputerStore;

/// The permitted total weight of cargo and installations (e.g. a roof rack) on top of the vehicle.\n\nTypical unit code(s): KGM for kilogram, LBR for pound\n\n* Note 1: You can indicate additional information in the [[name]] of the [[QuantitativeValue]] node.\n* Note 2: You may also link to a [[QualitativeValue]] node that provides additional information using [[valueReference]]\n* Note 3: Note that you can use [[minValue]] and [[maxValue]] to indicate ranges.
struct roofLoad;

/// A member of this organization.
struct members;

/// Enumerated status values for Reservation.
struct ReservationStatusType;

/// The number to access the service by text message.
struct serviceSmsNumber;

/// The vasculature that the vein drains into.
struct drainsTo;

/// Pathogenic virus that causes viral infection.
struct Virus;

/// A men's clothing store.
struct MensClothingStore;

/// A retail good store.
struct Store;

/// Defines the week(s) of the month on which a recurring Event takes place. Specified as an Integer between 1-5. For clarity, byMonthWeek is best used in conjunction with byDay to indicate concepts like the first and third Mondays of a month.
struct byMonthWeek;

/// A class, also often called a 'Type'; equivalent to rdfs:Class.
struct Class;

/// The point-in-time at which the provided description of the legislation is valid (e.g. : when looking at the law on the 2016-04-07 (= dateVersion), I get the consolidation of 2015-04-12 of the "National Insurance Contributions Act 2015")
struct legislationDateVersion;

/// SpokenWordAlbum.
struct SpokenWordAlbum;

/// The WebSite or SoftwareApplication where the interactions took place.
struct interactionService;

/// Design models for observational medical studies. Enumerated type.
struct MedicalObservationalStudyDesign;

/// An apartment (in American English) or flat (in British English) is a self-contained housing unit (a type of residential real estate) that occupies only part of a building (Source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Apartment">http://en.wikipedia.org/wiki/Apartment</a>).
struct Apartment;

/// A single feed providing structured information about one or more entities or topics.
struct DataFeed;

/// Represents the subscription pricing component of the total price for an offered product
struct Subscription;

/// The character of a medical substance, typically a medicine, of being available over the counter or not.
struct OTC;

/// Information about disease prevention.
struct diseasePreventionInfo;

/// Proficiency needed for this content; expected values: 'Beginner', 'Expert'.
struct proficiencyLevel;

/// Target audiences types for medical web pages. Enumerated type.
struct MedicalAudienceType;

/// A minimum amount that has to be paid in every month.
struct accountMinimumInflow;

/// Component dependency requirements for application. This includes runtime environments and shared libraries that are not included in the application distribution package, but required to run the application (Examples: DirectX, Java or .NET runtime).
struct requirements;

/// Nonprofit501c2: Non-profit type referring to Title-holding Corporations for Exempt Organizations.
struct Nonprofit501c2;

/// The type of fuel suitable for the engine or engines of the vehicle. If the vehicle has only one engine, this property can be attached directly to the vehicle.
struct fuelType;

/// The purpose of a work in the context of education; for example, 'assignment', 'group work'.
struct educationalUse;

/// A brand is a name used by an organization or business person for labeling a product, product group, or similar.
struct Brand;

/// Season dedicated to TV broadcast and associated online delivery.
struct TVSeason;

/// Enumerates the EU energy efficiency classes A-G as well as A+, A++, and A+++ as defined in EU directive 2017/1369
struct EUEnergyEfficiencyEnumeration;

/// (editorial work in progress, this definition is incomplete and unreviewed) MediaManipulationRatingEnumeration classifies a number of ways in which a media item (video, image, audio) can be manipulated, taking into account the context within which they are published or presented.
struct MediaManipulationRatingEnumeration;

/// A sub property of location. The specific food establishment where the action occurred.
struct foodEstablishment;

/// The act of editing a recipient by replacing an old object with a new object.
struct ReplaceAction;

/// A group of multiple reservations with common values for all sub-reservations.
struct ReservationPackage;

/// A winery.
struct Winery;

/// A file containing slides or used for a presentation.
struct PresentationDigitalDocument;

/// Label to match an [[OfferShippingDetails]] with a [[ShippingRateSettings]] (within the context of a [[shippingSettingsLink]] cross-reference).
struct shippingLabel;

/// The act of notifying someone of information pertinent to them, with no expectation of a response.
struct InformAction;

/// A service for a vehicle for hire with a driver for local travel. Fares are usually calculated based on distance traveled.
struct TaxiService;

/// A compound price specification is one that bundles multiple prices that all apply in combination for different dimensions of consumption. Use the name property of the attached unit price specification for indicating the dimension of a price component (e.g. "electricity" or "final cleaning").
struct CompoundPriceSpecification;

/// The publishing division which published the comic.
struct publisherImprint;

/// The act of expressing a difference of opinion with the object. An agent disagrees to/about an object (a proposition, topic or theme) with participants.
struct DisagreeAction;

/// Publication date of an online listing.
struct datePosted;

/// A jewelry store.
struct JewelryStore;

/// The availability of this item&#x2014;for example In stock, Out of stock, Pre-order, etc.
struct availability;

/// Identifier of the flight's arrival gate.
struct arrivalGate;

/// A dosage form in which this drug/supplement is available, e.g. 'tablet', 'suspension', 'injection'.
struct dosageForm;

/// A canal, like the Panama Canal.
struct Canal;

/// A common pathway for the electrochemical nerve impulses that are transmitted along each of the axons.
struct Nerve;

/// A seating map.
struct SeatingMap;

/// A video game series.
struct VideoGameSeries;

/// A short textual code (also called "store code") that uniquely identifies a place of business. The code is typically assigned by the parentOrganization and used in structured URLs.\n\nFor example, in the URL http://www.starbucks.co.uk/store-locator/etc/detail/3047 the code "3047" is a branchCode for a particular branch.
///       
struct branchCode;

/// Player type required&#x2014;for example, Flash or Silverlight.
struct playerType;

/// The latest someone may check out of a lodging establishment.
struct checkoutTime;

/// A hardware store.
struct HardwareStore;

/// A loan in which property or real estate is used as collateral. (A loan securitized against some real estate.)
struct MortgageLoan;

/// A sub property of object. The object that replaces.
struct replacer;

/// The delivery of the parcel related to this order or order item.
struct orderDelivery;

/// Quantity: Duration (use [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601)).
struct Duration;

/// A system of medicine that originated in India over thousands of years and that focuses on integrating and balancing the body, mind, and spirit.
struct Ayurvedic;

/// The lowest price of all offers available.\n\nUsage guidelines:\n\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
struct lowPrice;

/// Play mode: MultiPlayer. Requiring or allowing multiple human players to play simultaneously.
struct MultiPlayer;

/// Skin assessment with clinical examination.
struct Skin;

/// URL of the item.
struct url;

/// A piece of sculpture.
struct Sculpture;

/// Characteristics of the population for which this is intended, or which typically uses it, e.g. 'adults'.
struct targetPopulation;

/// The female gender.
struct Female;

/// Nonprofit501c3: Non-profit type referring to Religious, Educational, Charitable, Scientific, Literary, Testing for Public Safety, to Foster National or International Amateur Sports Competition, or Prevention of Cruelty to Children or Animals Organizations.
struct Nonprofit501c3;

/// Quantitative measure of the physiologic output of the exercise; also referred to as energy expenditure.
struct workload;

/// The act of rejecting to/adopting an object.\n\nRelated actions:\n\n* [[AcceptAction]]: The antonym of RejectAction.
struct RejectAction;

/// Instructions that explain how to achieve a result by performing a sequence of steps.
struct HowTo;

/// The size of the accommodation, e.g. in square meter or squarefoot.
/// Typical unit code(s): MTK for square meter, FTK for square foot, or YDK for square yard 
struct floorSize;

/// The timezone in [ISO 8601 format](http://en.wikipedia.org/wiki/ISO_8601) for which the service bases its broadcasts
struct broadcastTimezone;

/// MerchantReturnUnlimitedWindow: there is an unlimited window for product returns.
struct MerchantReturnUnlimitedWindow;

/// A self-storage facility.
struct SelfStorage;

/// A designation by the US FDA signifying that animal reproduction studies have failed to demonstrate a risk to the fetus and there are no adequate and well-controlled studies in pregnant women.
struct FDAcategoryB;

/// A structured representation of food or drink items available from a FoodEstablishment.
struct Menu;

/// A specific branch of medical science that studies the nerves and nervous system and its respective disease states.
struct Neurologic;

/// Indicates that a legislation is in force.
struct InForce;

/// The status of a confirmed reservation.
struct ReservationConfirmed;

/// The payment is due and considered late.
struct PaymentPastDue;

/// The anatomical or organ system that the vein flows into; a larger structure that the vein connects to.
struct tributary;

/// The closing hour of the place or service on the given day(s) of the week.
struct closes;

/// The height of the item.
struct height;

/// The blood vessel that carries blood from the heart to the muscle.
struct bloodSupply;

/// The model of the product. Use with the URL of a ProductModel or a textual representation of the model identifier. The URL of the ProductModel can be from an external source. It is recommended to additionally provide strong product identifiers via the gtin8/gtin13/gtin14 and mpn properties.
struct model;

/// The amount of money to pay in a single payment.
struct loanPaymentAmount;

/// Family name. In the U.S., the last name of a Person.
struct familyName;

/// CompilationAlbum.
struct CompilationAlbum;

/// If this NewsArticle appears in print, this field indicates the print section in which the article appeared.
struct printSection;

/// Whether the copay is before or after deductible, etc. TODO: Is this a closed set?
struct healthPlanCopayOption;

/// A subscription which allows a user to access media including audio, video, books, etc.
struct MediaSubscription;

/// An organization such as a school, NGO, corporation, club, etc.
struct Organization;

/// Data type: URL.
struct URL;

/// Nonprofit501c26: Non-profit type referring to State-Sponsored Organizations Providing Health Coverage for High-Risk Individuals.
struct Nonprofit501c26;

/// The individual who adds lettering, including speech balloons and sound effects, to artwork.
struct letterer;

/// Overview of the content. Contains a summarized view of the topic with the most relevant information for an introduction.
struct OverviewHealthAspect;

/// Anatomical features that can be observed by sight (without dissection), including the form and proportions of the human body as well as surface landmarks that correspond to deeper subcutaneous structures. Superficial anatomy plays an important role in sports medicine, phlebotomy, and other medical specialties as underlying anatomical structures can be identified through surface palpation. For example, during back surgery, superficial anatomy can be used to palpate and count vertebrae to find the site of incision. Or in phlebotomy, superficial anatomy can be used to locate an underlying vein; for example, the median cubital vein can be located by palpating the borders of the cubital fossa (such as the epicondyles of the humerus) and then looking for the superficial signs of the vein, such as size, prominence, ability to refill after depression, and feel of surrounding tissue support. As another example, in a subluxation (dislocation) of the glenohumeral joint, the bony structure becomes pronounced with the deltoid muscle failing to cover the glenohumeral joint allowing the edges of the scapula to be superficially visible. Here, the superficial anatomy is the visible edges of the scapula, implying the underlying dislocation of the joint (the related anatomical structure).
struct SuperficialAnatomy;

/// A media object, such as an image, video, or audio object embedded in a web page or a downloadable dataset i.e. DataDownload. Note that a creative work may have many media objects associated with it on the same web page. For example, a page about a single song (MusicRecording) may have a music video (VideoObject), and a high and low bandwidth audio stream (2 AudioObject's).
struct MediaObject;

/// The priority status assigned to a passenger for security or boarding (e.g. FastTrack or Priority).
struct passengerPriorityStatus;

/// The number of grams of unsaturated fat.
struct unsaturatedFatContent;

/// For an [[Organization]] (e.g. [[NewsMediaOrganization]]), a statement describing (in news media, the newsroom’s) disclosure and correction policy for errors.
struct correctionsPolicy;

/// The count of total number of reviews.
struct reviewCount;

/// OrderStatus representing that payment is due on an order.
struct OrderPaymentDue;

/// Identifies the issue of publication; for example, "iii" or "2".
struct issueNumber;

/// The currency (in 3-letter of the drug cost. See: http://en.wikipedia.org/wiki/ISO_4217 
struct costCurrency;

/// An enterprise (potentially individual but typically collaborative), planned to achieve a particular aim.
/// Use properties from [[Organization]], [[subOrganization]]/[[parentOrganization]] to indicate project sub-structures. 
///    
struct Project;

/// A sub property of participant. The opponent on this action.
struct opponent;

/// A post to a social media platform, including blog posts, tweets, Facebook posts, etc.
struct SocialMediaPosting;

/// The postal code. For example, 94043.
struct postalCode;

/// The amount to be paid as a penalty in the event of early payment of the loan.
struct earlyPrepaymentPenalty;

/// A music recording (track)&#x2014;usually a single song.
struct tracks;

/// A branch of medicine that is involved in the dental care.
struct Dentistry;

/// The highest price of all offers available.\n\nUsage guidelines:\n\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similiar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
struct highPrice;

/// This ordering relation for qualitative values indicates that the subject is greater than the object.
struct greater;

/// A link related to this web page, for example to other related web pages.
struct relatedLink;

/// SoundtrackAlbum.
struct SoundtrackAlbum;

/// The act of  departing from a place. An agent departs from an fromLocation for a destination, optionally with participants.
struct DepartAction;

/// A floor limit is the amount of money above which credit card transactions must be authorized.
struct floorLimit;

/// Indicates whether this drug is available by prescription or over-the-counter.
struct DrugPrescriptionStatus;

/// A pointer to another, functionally similar product (or multiple products).
struct isSimilarTo;

/// The CreativeWork that captured all or part of this Event.
struct recordedIn;

/// Indicates that the item has been discontinued.
struct Discontinued;

/// Attraction suitable for type(s) of tourist. eg. Children, visitors from a particular country, etc. 
struct touristType;

/// A pointer to another, somehow related product (or multiple products).
struct isRelatedTo;

/// The status of the study (enumerated).
struct status;

/// Indicates a [[Product]] that is a member of this [[ProductGroup]] (or [[ProductModel]]).
struct hasVariant;

/// The Organization responsible for authenticating the user's subscription. For example, many media apps require a cable/satellite provider to authenticate your subscription before playing media.
struct authenticator;

/// Information about actions or measures that can be taken to avoid getting the topic or reaching a critical situation related to the topic.
struct PreventionHealthAspect;

/// The costs of settling the payment using a particular payment method.
struct PaymentChargeSpecification;

/// The currency accepted.\n\nUse standard formats: [ISO 4217 currency format](http://en.wikipedia.org/wiki/ISO_4217) e.g. "USD"; [Ticker symbol](https://en.wikipedia.org/wiki/List_of_cryptocurrencies) for cryptocurrencies e.g. "BTC"; well known names for [Local Exchange Tradings Systems](https://en.wikipedia.org/wiki/Local_exchange_trading_system) (LETS) and other currency types e.g. "Ithaca HOUR".
struct currenciesAccepted;

/// A stadium.
struct StadiumOrArena;

/// Represents EU Energy Efficiency Class C as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryC;

/// Destination address.
struct deliveryAddress;

/// ReturnFeesEnumeration expresses policies for return fees.
struct ReturnFeesEnumeration;

/// The most generic bi-directional social/work relation.
struct knows;

/// The stop or station from which the bus departs.
struct departureBusStop;

/// The stepValue attribute indicates the granularity that is expected (and required) of the value in a PropertyValueSpecification.
struct stepValue;

/// The payment is due, but still within an acceptable time to be received.
struct PaymentDue;

/// A medical device used for therapeutic purposes.
struct Therapeutic;

/// Upcoming or past event associated with this place, organization, or action.
struct event;

/// A registry-based study design.
struct Registry;

/// Represents EU Energy Efficiency Class A+ as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryA1Plus;

/// Description of bonus and commission compensation aspects of the job.
struct incentives;

/// Lists or enumerations dealing with status types.
struct StatusEnumeration;

/// Indicates that the CreativeWork contains a reference to, but is not necessarily about a concept.
struct mentions;

/// OrderStatus representing successful delivery of an order.
struct OrderDelivered;

/// Whether the legislation is currently in force, not in force, or partially in force.
struct legislationLegalForce;

/// The audience eligible for this service.
struct serviceAudience;

/// Represents spatial relations in which two geometries (or the places they represent) touch: they have at least one boundary point in common, but no interior points." (a symmetric relationship, as defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM) )
struct geoTouches;

/// An available dosage strength for the drug.
struct availableStrength;

/// The operating organization, if different from the provider.  This enables the representation of services that are provided by an organization, but operated by another organization like a subcontractor.
struct serviceOperator;

/// The permitted vertical load (TWR) of a trailer attached to the vehicle. Also referred to as Tongue Load Rating (TLR) or Vertical Load Rating (VLR)\n\nTypical unit code(s): KGM for kilogram, LBR for pound\n\n* Note 1: You can indicate additional information in the [[name]] of the [[QuantitativeValue]] node.\n* Note 2: You may also link to a [[QualitativeValue]] node that provides additional information using [[valueReference]].\n* Note 3: Note that you can use [[minValue]] and [[maxValue]] to indicate ranges.
struct tongueWeight;

/// Associates an [[Event]] with a [[Schedule]]. There are circumstances where it is preferable to share a schedule for a series of
///       repeating events rather than data on the individual events themselves. For example, a website or application might prefer to publish a schedule for a weekly
///       gym class rather than provide data on every event. A schedule could be processed by applications to add forthcoming events to a calendar. An [[Event]] that
///       is associated with a [[Schedule]] using this property should not have [[startDate]] or [[endDate]] properties. These are instead defined within the associated
///       [[Schedule]], this avoids any ambiguity for clients using the data. The property might have repeated values to specify different schedules, e.g. for different months
///       or seasons.
struct eventSchedule;

/// A colleague of the person.
struct colleague;

/// A description of any security clearance requirements of the job.
struct securityClearanceRequirement;

/// A trip on a commercial ferry line.
struct BoatTrip;

/// The number of grams of trans fat.
struct transFatContent;

/// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
struct potentialAction;

/// Email address.
struct email;

/// The number of grams of protein.
struct proteinContent;

/// A sub property of participant. The person/organization being supported.
struct endorsee;

/// A creative work that this work is an example/instance/realization/derivation of.
struct exampleOfWork;

/// The act of traveling from an fromLocation to a destination by a specified mode of transport, optionally with participants.
struct TravelAction;

/// The number of positions open for this job posting. Use a positive integer. Do not use if the number of positions is unclear or not known.
struct totalJobOpenings;

/// A single season of a podcast. Many podcasts do not break down into separate seasons. In that case, PodcastSeries should be used.
struct PodcastSeason;

/// A value indicating a steering position.
struct SteeringPositionValue;

/// CharitableIncorporatedOrganization: Non-profit type referring to a Charitable Incorporated Organization (UK).
struct CharitableIncorporatedOrganization;

/// An EndorsementRating is a rating that expresses some level of endorsement, for example inclusion in a "critic's pick" blog, a
/// "Like" or "+1" on a social network. It can be considered the [[result]] of an [[EndorseAction]] in which the [[object]] of the action is rated positively by
/// some [[agent]]. As is common elsewhere in schema.org, it is sometimes more useful to describe the results of such an action without explicitly describing the [[Action]].
/// 
/// An [[EndorsementRating]] may be part of a numeric scale or organized system, but this is not required: having an explicit type for indicating a positive,
/// endorsement rating is particularly useful in the absence of numeric scales as it helps consumers understand that the rating is broadly positive.
struct EndorsementRating;

/// The total integer number of bedrooms in a some [[Accommodation]], [[ApartmentComplex]] or [[FloorPlan]].
struct numberOfBedrooms;

/// All the documents published by an official publisher should have at least the legal value level "OfficialLegalValue". This indicates that the document was published by an organisation with the public task of making it available (e.g. a consolidated version of a EU directive published by the EU Office of Publications).
struct OfficialLegalValue;

/// Current location of the item.
struct itemLocation;

/// The elevation of a location ([WGS 84](https://en.wikipedia.org/wiki/World_Geodetic_System)). Values may be of the form 'NUMBER UNIT_OF_MEASUREMENT' (e.g., '1,000 m', '3,200 ft') while numbers alone should be assumed to be a value in meters.
struct elevation;

/// An honorific prefix preceding a Person's name such as Dr/Mrs/Mr.
struct honorificPrefix;

/// Level of evidence for a medical guideline. Enumerated type.
struct MedicalEvidenceLevel;

/// Example/instance/realization/derivation of the concept of this creative work. eg. The paperback edition, first edition, or eBook.
struct workExample;

/// Runtime platform or script interpreter dependencies (Example - Java v1, Python2.3, .Net Framework 3.0).
struct runtimePlatform;

/// A part of a successively published publication such as a periodical or multi-volume work, often numbered. It may represent a time span, such as a year.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
struct PublicationVolume;

/// A travel agency.
struct TravelAgency;

/// Indicates (by URL or string) a particular version of a schema used in some CreativeWork. For example, a document could declare a schemaVersion using an URL such as https://schema.org/version/2.0/ if precise indication of schema version was required by some application. 
struct schemaVersion;

/// The average rating based on multiple ratings or reviews.
struct AggregateRating;

/// Positron emission tomography imaging.
struct PET;

/// OneTimePayments: this is a benefit for one-time payments for individuals.
struct OneTimePayments;

/// One or more detailed price specifications, indicating the unit price and delivery or payment charges.
struct priceSpecification;

/// Device required to run the application. Used in cases where a specific make/model is required to run the application.
struct device;

/// The total time required to perform instructions or a direction (including time to prepare the supplies), in [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601).
struct totalTime;

/// The date when the item becomes valid.
struct validFrom;

/// Indicates the populationType common to all members of a [[StatisticalPopulation]].
struct populationType;

/// Video game which is played on this server.
struct game;

/// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use [[businessFunction]] to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a [[Demand]]. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
///       
struct offers;

/// The type of service being offered, e.g. veterans' benefits, emergency relief, etc.
struct serviceType;

/// The number of answers this question has received.
struct answerCount;

/// An HTTP method that specifies the appropriate HTTP method for a request to an HTTP EntryPoint. Values are capitalized strings as used in HTTP.
struct httpMethod;

/// A tourist attraction.  In principle any Thing can be a [[TouristAttraction]], from a [[Mountain]] and [[LandmarksOrHistoricalBuildings]] to a [[LocalBusiness]].  This Type can be used on its own to describe a general [[TouristAttraction]], or be used as an [[additionalType]] to add tourist attraction properties to any other type.  (See examples below)
struct TouristAttraction;

/// A cafe or coffee shop.
struct CafeOrCoffeeShop;

/// Information about getting tested (for a [[MedicalCondition]]), e.g. in the context of a pandemic.
struct gettingTestedInfo;

/// A number that confirms the given order or payment has been received.
struct confirmationNumber;

/// A single ingredient used in the recipe, e.g. sugar, flour or garlic.
struct ingredients;

/// A release of this album.
struct albumRelease;

/// The location where the CreativeWork was created, which may not be the same as the location depicted in the CreativeWork.
struct locationCreated;

/// The number of the column in which the NewsArticle appears in the print edition.
struct printColumn;

/// A performer at the event&#x2014;for example, a presenter, musician, musical group or actor.
struct performer;

/// A TV episode which can be part of a series or season.
struct TVEpisode;

/// X-ray imaging.
struct XRay;

/// A parking lot or other parking facility.
struct ParkingFacility;

/// The overall order the items in this delivery were included in.
struct partOfOrder;

/// An adult entertainment establishment.
struct AdultEntertainment;

/// A downloadable form of this dataset, at a specific location, in a specific format.
struct distribution;

/// Defines the type of a price specified for an offered product, for example a list price, a (temporary) sale price or a manufacturer suggested retail price. If multiple prices are specified for an offer the [[priceType]] property can be used to identify the type of each such specified price. The value of priceType can be specified as a value from enumeration PriceTypeEnumeration or as a free form text string for price types that are not already predefined in PriceTypeEnumeration.
struct priceType;

/// Classification of the album by it's type of content: soundtrack, live album, studio album, etc.
struct MusicAlbumProductionType;

/// A category describing the job, preferably using a term from a taxonomy such as [BLS O*NET-SOC](http://www.onetcenter.org/taxonomy.html), [ISCO-08](https://www.ilo.org/public/english/bureau/stat/isco/isco08/) or similar, with the property repeated for each applicable value. Ideally the taxonomy should be identified, and both the textual label and formal code for the category should be provided.\n
/// Note: for historical reasons, any textual label and formal code provided as a literal may be assumed to be from O*NET-SOC.
struct occupationalCategory;

/// The vasculature the lymphatic structure originates, or afferents, from.
struct originatesFrom;

/// A series of books. Included books can be indicated with the hasPart property.
struct BookSeries;

/// The endTime of something. For a reserved event or service (e.g. FoodEstablishmentReservation), the time that it is expected to end. For actions that span a period of time, when the action was performed. e.g. John wrote a book from January to *December*. For media, including audio and video, it's the time offset of the end of a clip within a larger file.\n\nNote that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
struct endTime;

/// A course or class that is one of the learning opportunities that constitute an educational / occupational program. No information is implied about whether the course is mandatory or optional; no guarantee is implied about whether the course will be available to everyone on the program.
struct hasCourse;

/// A list of possible conditions for the item.
struct OfferItemCondition;

/// The language of the content or performance or used in an action. Please use one of the language codes from the [IETF BCP 47 standard](http://tools.ietf.org/html/bcp47). See also [[availableLanguage]].
struct inLanguage;

/// A sub property of location. The sports activity location where this action occurred.
struct sportsActivityLocation;

/// A type of sport (e.g. Baseball).
struct sport;

/// The number of children staying in the unit.
struct numChildren;

/// A full description of the lodging unit.
struct lodgingUnitDescription;

/// The kind of release which this album is: single, EP or album.
struct albumReleaseType;

/// The date at which the program stops collecting applications for the next enrollment cycle.
struct applicationDeadline;

/// DisabilitySupport: this is a benefit for disability support.
struct DisabilitySupport;

/// The country of the principal offices of the production company or individual responsible for the movie or program.
struct countryOfOrigin;

/// The edition of the book.
struct bookEdition;

/// A specific branch of medical science that specializes in the care of infants, children and adolescents.
struct Pediatric;

/// A music store.
struct MusicStore;

/// 'bookingAgent' is an out-dated term indicating a 'broker' that serves as a booking agent.
struct bookingAgent;

/// The act of distributing content to people for their amusement or edification.
struct ShareAction;

/// A sub property of location. The entertainment business where the action occurred.
struct entertainmentBusiness;

/// Cheat codes to the game.
struct cheatCode;

/// A specific branch of medical science that deals with the evaluation and initial treatment of medical conditions caused by trauma or sudden illness.
struct Emergency;

/// A short radio program or a segment/part of a radio program.
struct RadioClip;

/// The production company or studio responsible for the item e.g. series, video game, episode etc.
struct productionCompany;

/// The number of adults staying in the unit.
struct numAdults;

/// The act of editing by adding an object to a collection.
struct AddAction;

/// A health club.
struct HealthClub;

/// The act of committing to/adopting an object.\n\nRelated actions:\n\n* [[RejectAction]]: The antonym of AcceptAction.
struct AcceptAction;

/// Gender of something, typically a [[Person]], but possibly also fictional characters, animals, etc. While https://schema.org/Male and https://schema.org/Female may be used, text strings are also acceptable for people who do not identify as a binary gender. The [[gender]] property can also be used in an extended sense to cover e.g. the gender of sports teams. As with the gender of individuals, we do not try to enumerate all possibilities. A mixed-gender [[SportsTeam]] can be indicated with a text value of "Mixed".
struct gender;

/// The beginning of the availability of the product or service included in the offer.
struct availabilityStarts;

/// Text value being annotated.
struct textValue;

/// A tennis complex.
struct TennisComplex;

/// Device used to perform the test.
struct usesDevice;

/// An account that allows an investor to deposit funds and place investment orders with a licensed broker or brokerage firm.
struct BrokerageAccount;

/// Rigid connective tissue that comprises up the skeletal structure of the human body.
struct Bone;

/// A spreadsheet file.
struct SpreadsheetDigitalDocument;

/// An indicator as to whether a position is available for an immediate start.
struct jobImmediateStart;

/// Additional content for a software application.
struct softwareAddOn;

/// A structured value representing the duration and scope of services that will be provided to a customer free of charge in case of a defect or malfunction of a product.
struct WarrantyPromise;

/// The date at which the program begins collecting applications for the next enrollment cycle.
struct applicationStartDate;

/// A meeting room, conference room, or conference hall is a room provided for singular events such as business conferences and meetings (Source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Conference_hall">http://en.wikipedia.org/wiki/Conference_hall</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct MeetingRoom;

/// A description of the job location (e.g TELECOMMUTE for telecommute jobs).
struct jobLocationType;

/// A general contractor.
struct GeneralContractor;

/// The identifier property represents any kind of identifier for any kind of [[Thing]], such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See [background notes](/docs/datamodel.html#identifierBg) for more details.
///         
struct identifier;

/// A waterfall, like Niagara.
struct Waterfall;

/// Bed and breakfast.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct BedAndBreakfast;

/// A type of medical procedure that involves noninvasive techniques.
struct NoninvasiveProcedure;

/// Date order was placed.
struct orderDate;

/// The Organization on whose behalf the creator was working.
struct sourceOrganization;

/// Nonprofit501c27: Non-profit type referring to State-Sponsored Workers' Compensation Reinsurance Organizations.
struct Nonprofit501c27;

/// Nonprofit501a: Non-profit type referring to Farmers’ Cooperative Associations.
struct Nonprofit501a;

/// A real-estate agent.
struct RealEstateAgent;

/// A gym.
struct ExerciseGym;

/// The datetime the item was removed from the DataFeed.
struct dateDeleted;

/// Short story or tale. A brief work of literature, usually written in narrative prose.
struct ShortStory;

/// The act of participating in an exchange of goods and services for monetary compensation. An agent trades an object, product or service with a participant in exchange for a one time or periodic payment.
struct TradeAction;

/// A marginOfError for an [[Observation]].
struct marginOfError;

/// One or more alternative conditions considered in the differential diagnosis process as output of a diagnosis process.
struct diagnosis;

/// A season that is part of the media series.
struct containsSeason;

/// A word, name, acronym, phrase, etc. with a formal definition. Often used in the context of category or subject classification, glossaries or dictionaries, product or creative work types, etc. Use the name property for the term being defined, use termCode if the term has an alpha-numeric code allocated, use description to provide the definition of the term.
struct DefinedTerm;

/// The act of an agent communicating (service provider, social media, etc) their arrival by registering/confirming for a previously reserved service (e.g. flight check in) or at a place (e.g. hotel), possibly resulting in a result (boarding pass, etc).\n\nRelated actions:\n\n* [[CheckOutAction]]: The antonym of CheckInAction.\n* [[ArriveAction]]: Unlike ArriveAction, CheckInAction implies that the agent is informing/confirming the start of a previously reserved service.\n* [[ConfirmAction]]: Unlike ConfirmAction, CheckInAction implies that the agent is informing/confirming the *start* of a previously reserved service rather than its validity/existence.
struct CheckInAction;

/// Original definition: "provider of professional services."\n\nThe general [[ProfessionalService]] type for local businesses was deprecated due to confusion with [[Service]]. For reference, the types that it included were: [[Dentist]],
///         [[AccountingService]], [[Attorney]], [[Notary]], as well as types for several kinds of [[HomeAndConstructionBusiness]]: [[Electrician]], [[GeneralContractor]],
///         [[HousePainter]], [[Locksmith]], [[Plumber]], [[RoofingContractor]]. [[LegalService]] was introduced as a more inclusive supertype of [[Attorney]].
struct ProfessionalService;

/// Type of app development: phone, Metro style, desktop, XBox, etc.
struct targetPlatform;

/// The action that takes in a math expression and directs users to a page potentially capable of solving/simplifying that expression.
struct SolveMathAction;

/// A recommended dosing schedule for a drug or supplement as prescribed or recommended by an authority or by the drug/supplement's manufacturer. Capture the recommending authority in the recognizingAuthority property of MedicalEntity.
struct RecommendedDoseSchedule;

/// A particular physical or virtual business of an organization for medical purposes. Examples of MedicalBusiness include differents business run by health professionals.
struct MedicalBusiness;

/// A sub property of object. A question.
struct question;

/// Physical activity that is of high-intensity which utilizes the anaerobic metabolism of the body.
struct AnaerobicActivity;

/// An OfferCatalog is an ItemList that contains related Offers and/or further OfferCatalogs that are offeredBy the same provider.
struct OfferCatalog;

/// A scholarly article.
struct ScholarlyArticle;

/// Specifies a regular expression for testing literal values according to the HTML spec.
struct valuePattern;

/// Other prominent or relevant topics tied to the main topic.
struct RelatedTopicsHealthAspect;

/// An event involving the delivery of an item.
struct DeliveryEvent;

/// The number of words in the text of the Article.
struct wordCount;

/// A MusicRelease is a specific release of a music album.
struct MusicRelease;

/// Relates a property to a class that constitutes (one of) the expected type(s) for values of the property.
struct rangeIncludes;

/// Brewery.
struct Brewery;

/// Identifier of the flight's arrival terminal.
struct arrivalTerminal;

/// The date the item e.g. vehicle was purchased by the current owner.
struct purchaseDate;

/// An organization with archival holdings. An organization which keeps and preserves archival material and typically makes it accessible to the public.
struct ArchiveOrganization;

/// The distance of the flight.
struct flightDistance;

/// A gas station.
struct GasStation;

/// Nonprofit501c24: Non-profit type referring to Section 4049 ERISA Trusts.
struct Nonprofit501c24;

/// The number of screens in the movie theater.
struct screenCount;

/// A tourist information center.
struct TouristInformationCenter;

/// Upcoming or past events associated with this place or organization.
struct events;

/// A modifiable or non-modifiable risk factor included in the calculation, e.g. age, coexisting condition.
struct includedRiskFactor;

/// Indicates when shipping to a particular [[shippingDestination]] is not available.
struct doesNotShip;

/// The program providing the membership.
struct programName;

/// The width of the item.
struct width;

/// A quotation. Often but not necessarily from some written work, attributable to a real world author and - if associated with a fictional character - to any fictional Person. Use [[isBasedOn]] to link to source/origin. The [[recordedIn]] property can be used to reference a Quotation from an [[Event]].
struct Quotation;

/// The value of an active ingredient's strength, e.g. 325.
struct strengthValue;

/// RsvpResponseType is an enumeration type whose instances represent responding to an RSVP request.
struct RsvpResponseType;

/// Event type: Social event.
struct SocialEvent;

/// Knowledge, skill, ability or personal attribute that must be demonstrated by a person or other entity in order to do something such as earn an Educational Occupational Credential or understand a LearningResource.
struct competencyRequired;

/// The industry associated with the job position.
struct industry;

/// A review of the item.
struct review;

/// Where a taxi will pick up a passenger or a rental car can be picked up.
struct pickupLocation;

/// Nonprofit501c11: Non-profit type referring to Teachers' Retirement Fund Associations.
struct Nonprofit501c11;

/// The subchannel used for the broadcast.
struct broadcastSubChannel;

/// A business that provide Heating, Ventilation and Air Conditioning services.
struct HVACBusiness;

/// The typical delay between the receipt of the order and the goods either leaving the warehouse or being prepared for pickup, in case the delivery method is on site pickup. Typical properties: minValue, maxValue, unitCode (d for DAY).  This is by common convention assumed to mean business days (if a unitCode is used, coded as "d"), i.e. only counting days when the business normally operates.
struct handlingTime;

/// The [Global Location Number](http://www.gs1.org/gln) (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
struct globalLocationNumber;

/// The act of granting permission to an object.
struct AuthorizeAction;

/// The artist that performed this album or recording.
struct byArtist;

/// The median value.
struct median;

/// A single episode of a podcast series.
struct PodcastEpisode;

/// Data derived from a single randomized trial, or nonrandomized studies.
struct EvidenceLevelB;

/// The terminal or port from which the boat arrives.
struct arrivalBoatTerminal;

/// The platform where the train arrives.
struct arrivalPlatform;

/// A service which provides access to media programming like TV or radio. Access may be via cable or satellite.
struct CableOrSatelliteService;

/// A television station.
struct TelevisionStation;

/// Format of this release (the type of recording media used, ie. compact disc, digital media, LP, etc.).
struct MusicReleaseFormatType;

/// The current status of the reservation.
struct reservationStatus;

/// The status for a previously confirmed reservation that is now cancelled.
struct ReservationCancelled;

/// The date/time the message was received if a single recipient exists.
struct dateReceived;

/// A therapy that duplicates or overlaps this one.
struct duplicateTherapy;

/// Textual description of the unit type (including suite vs. room, size of bed, etc.).
struct lodgingUnitType;

/// The biomechanical properties of the bone.
struct biomechnicalClass;

/// A subclass of Role used to describe roles within organizations.
struct OrganizationRole;

/// The act of swallowing solid objects.
struct EatAction;

/// A contact point for a person or organization.
struct contactPoint;

/// The type of component used for transmitting the power from a rotating power source to the wheels or other relevant component(s) ("gearbox" for cars).
struct vehicleTransmission;

/// The most significant URLs on the page. Typically, these are the non-navigation links that are clicked on the most.
struct significantLinks;

/// The street address. For example, 1600 Amphitheatre Pkwy.
struct streetAddress;

/// UKTrust: Non-profit type referring to a UK trust.
struct UKTrust;

/// Used to describe membership in a loyalty programs (e.g. "StarAliance"), traveler clubs (e.g. "AAA"), purchase clubs ("Safeway Club"), etc.
struct ProgramMembership;

/// The lowest price if the price is a range.
struct minPrice;

/// A creative work with a visual storytelling format intended to be viewed online, particularly on mobile devices.
struct AmpStory;

/// A blog.
struct Blog;

/// Represents EU Energy Efficiency Class A as defined in EU energy labeling regulations
struct EUEnergyEfficiencyCategoryA;

/// The method of cooking, such as Frying, Steaming, ...
struct cookingMethod;

/// The location depicted or described in the content. For example, the location in a photograph or painting.
struct contentLocation;

/// Either the actual menu as a structured representation, as text, or a URL of the menu.
struct hasMenu;

/// Destination(s) ( [[Place]] ) that make up a trip. For a trip where destination order is important use [[ItemList]] to specify that order (see examples).
struct itinerary;

/// Bank or credit union.
struct BankOrCreditUnion;

/// Used to indicate whether a product is EnergyStar certified
struct EnergyStarEnergyEfficiencyEnumeration;

/// A shoe store.
struct ShoeStore;

/// The name of the train (e.g. The Orient Express).
struct trainName;

/// The type/class of the seat.
struct seatingType;

/// A medical procedure intended primarily for palliative purposes, aimed at relieving the symptoms of an underlying health condition.
struct PalliativeProcedure;

/// A fire station. With firemen.
struct FireStation;

/// A medical procedure involving an incision with instruments; performed for diagnose, or therapeutic purposes.
struct SurgicalProcedure;

/// The act of inserting at the beginning if an ordered collection.
struct PrependAction;

/// A trip on a commercial train line.
struct TrainTrip;

/// A license document that applies to this content, typically indicated by URL.
struct license;

/// The estimated time the flight will take.
struct estimatedFlightDuration;

/// The duration of the warranty promise. Common unitCode values are ANN for year, MON for months, or DAY for days.
struct durationOfWarranty;

/// [[Guide]] is a page or article that recommend specific products or services, or aspects of a thing for a user to consider. A [[Guide]] may represent a Buying Guide and detail aspects of products or services for a user to consider. A [[Guide]] may represent a Product Guide and recommend specific products or services. A [[Guide]] may represent a Ranked List and recommend specific products or services with ranking.
struct Guide;

/// The act of swallowing liquids.
struct DrinkAction;

/// A vehicle is a device that is designed or used to transport people or cargo over land, water, air, or through space.
struct Vehicle;

/// The Action representing the type of interaction. For up votes, +1s, etc. use [[LikeAction]]. For down votes use [[DislikeAction]]. Otherwise, use the most specific Action.
struct interactionType;

/// The unique identifier for the ticket.
struct ticketNumber;

/// A sub property of participant. The person that borrows the object being lent.
struct borrower;

/// A financial aid type or program which students may use to pay for tuition or fees associated with the program.
struct financialAidEligible;

/// The act of forming a personal connection with someone/something (object) unidirectionally/asymmetrically to get updates pushed to.\n\nRelated actions:\n\n* [[FollowAction]]: Unlike FollowAction, SubscribeAction implies that the subscriber acts as a passive agent being constantly/actively pushed for updates.\n* [[RegisterAction]]: Unlike RegisterAction, SubscribeAction implies that the agent is interested in continuing receiving updates from the object.\n* [[JoinAction]]: Unlike JoinAction, SubscribeAction implies that the agent is interested in continuing receiving updates from the object.
struct SubscribeAction;

/// The target audience for this permit.
struct permitAudience;

/// An instance of a [[Course]] which is distinct from other instances because it is offered at a different time or location or through different media or modes of study or to a specific section of students.
struct CourseInstance;

/// Points-of-Sales operated by the organization or person.
struct hasPOS;

/// The act of marrying a person.
struct MarryAction;

/// Side effects that can be observed from the usage of the topic.
struct SideEffectsHealthAspect;

/// The kind of release which this album is: single, EP or album.
struct MusicAlbumReleaseType;

/// Represents the minimum advertised price ("MAP") (as dictated by the manufacturer) of an offered product.
struct MinimumAdvertisedPrice;

/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
struct UserPlusOnes;

/// A graveyard.
struct Cemetery;

/// Withdrawn.
struct Withdrawn;

/// The measuredValue of an [[Observation]].
struct measuredValue;

/// The airline boards by zones of the plane.
struct ZoneBoardingPolicy;

/// A [[HyperTocEntry]] can have a [[tocContinuation]] indicated, which is another [[HyperTocEntry]] that would be the default next item to play or render.
struct tocContinuation;

/// A bridge.
struct Bridge;

/// Identifies the volume of publication or multi-part work; for example, "iii" or "2".
struct volumeNumber;

/// A [[comment]] that corrects [[CreativeWork]].
struct CorrectionComment;

/// A theater or other performing art center.
struct PerformingArtsTheater;

/// The movie presented during this event.
struct workPresented;

/// The number of episodes in this season or series.
struct numberOfEpisodes;

/// A season in a media series.
struct season;

/// Minimum memory requirements.
struct memoryRequirements;

/// Type(s) of exercise or activity, such as strength training, flexibility training, aerobics, cardiac rehabilitation, etc.
struct exerciseType;

/// A StoreCreditRefund ...
struct StoreCreditRefund;

/// Nonprofit501f: Non-profit type referring to Cooperative Service Organizations.
struct Nonprofit501f;

/// The delivery method(s) available for this offer.
struct availableDeliveryMethod;

/// Date of death.
struct deathDate;

/// Represents the suggested retail price ("SRP") of an offered product.
struct SRP;

/// The act of registering to be a user of a service, product or web page.\n\nRelated actions:\n\n* [[JoinAction]]: Unlike JoinAction, RegisterAction implies you are registering to be a user of a service, *not* a group/team of people.\n* [FollowAction]]: Unlike FollowAction, RegisterAction doesn't imply that the agent is expecting to poll for updates from the object.\n* [[SubscribeAction]]: Unlike SubscribeAction, RegisterAction doesn't imply that the agent is expecting updates from the object.
struct RegisterAction;

/// The interval and unit of measurement of ordering quantities for which the offer or price specification is valid. This allows e.g. specifying that a certain freight charge is valid only for a certain quantity.
struct eligibleQuantity;

/// A bus stop.
struct BusStop;

/// A Buddhist temple.
struct BuddhistTemple;

/// A statistical distribution of monetary amounts.
struct MonetaryAmountDistribution;

/// An agent leaves an event / group with participants/friends at a location.\n\nRelated actions:\n\n* [[JoinAction]]: The antonym of LeaveAction.\n* [[UnRegisterAction]]: Unlike UnRegisterAction, LeaveAction implies leaving a group/team of people rather than a service.
struct LeaveAction;

/// The airline boards by groups based on check-in time, priority, etc.
struct GroupBoardingPolicy;

/// The act of manipulating/administering/supervising/controlling one or more objects.
struct OrganizeAction;

/// A sub-property of instrument. A supply consumed when performing instructions or a direction.
struct supply;

/// A parent of this person.
struct parent;

/// Where a rental car can be dropped off.
struct dropoffLocation;

/// Defines a [[Date]] or [[DateTime]] during which a scheduled [[Event]] will not take place. The property allows exceptions to
///       a [[Schedule]] to be specified. If an exception is specified as a [[DateTime]] then only the event that would have started at that specific date and time
///       should be excluded from the schedule. If an exception is specified as a [[Date]] then any event that is scheduled for that 24 hour period should be
///       excluded from the schedule. This allows a whole day to be excluded from the schedule without having to itemise every scheduled event.
struct exceptDate;

/// Indicates a MediaManipulationRatingEnumeration classification of a media object (in the context of how it was published or shared).
struct mediaAuthenticityCategory;

/// A NewsArticle is an article whose content reports news, or provides background context and supporting materials for understanding the news.
/// 
/// A more detailed overview of [schema.org News markup](/docs/news.html) is also available.
struct NewsArticle;

/// An Organization (or ProgramMembership) to which this Person or Organization belongs.
struct memberOf;

/// A route by which this drug may be administered, e.g. 'oral'.
struct administrationRoute;

/// The legal availability status of a medical drug.
struct DrugLegalStatus;

/// The interest rate, charged or paid, applicable to the financial product. Note: This is different from the calculated annualPercentageRate.
struct interestRate;

/// A US-style health insurance plan, including PPOs, EPOs, and HMOs. 
struct HealthInsurancePlan;

/// An organization that provides flights for passengers.
struct Airline;

/// Indicates the name of the PropertyValueSpecification to be used in URL templates and form encoding in a manner analogous to HTML's input@name.
struct valueName;

/// An action that failed to complete. The action's error property and the HTTP return code contain more information about the failure.
struct FailedActionStatus;

/// A list of possible product availability options.
struct ItemAvailability;

/// Indicates that the item is available for ordering and delivery before general availability.
struct PreSale;

/// Enumerations related to health and the practice of medicine: A concept that is used to attribute a quality to another concept, as a qualifier, a collection of items or a listing of all of the elements of a set in medicine practice.
struct MedicalEnumeration;

/// OrderStatus representing that there is a problem with the order.
struct OrderProblem;

/// The act of expressing a positive sentiment about the object. An agent likes an object (a proposition, topic or theme) with participants.
struct LikeAction;

/// Related anatomical structure(s) that are not part of the system but relate or connect to it, such as vascular bundles associated with an organ system.
struct relatedStructure;

/// The total distance travelled by the particular vehicle since its initial production, as read from its odometer.\n\nTypical unit code(s): KMT for kilometers, SMI for statute miles
struct mileageFromOdometer;

/// A work featured in some event, e.g. exhibited in an ExhibitionEvent.
///        Specific subproperties are available for workPerformed (e.g. a play), or a workPresented (a Movie at a ScreeningEvent).
struct workFeatured;

/// Medical researchers.
struct MedicalResearcher;

/// Whether the property must be filled in to complete the action.  Default is false.
struct valueRequired;

/// The item ordered.
struct orderedItem;

/// Associated product/technology version. e.g., .NET Framework 4.5.
struct assemblyVersion;

/// A media episode (e.g. TV, radio, video game) which can be part of a series or season.
struct Episode;

/// A broadcast channel of a broadcast service.
struct hasBroadcastChannel;

/// A flag to signal that the item, event, or place is accessible for free.
struct free;

/// The permitted total weight of the loaded vehicle, including passengers and cargo and the weight of the empty vehicle.\n\nTypical unit code(s): KGM for kilogram, LBR for pound\n\n* Note 1: You can indicate additional information in the [[name]] of the [[QuantitativeValue]] node.\n* Note 2: You may also link to a [[QualitativeValue]] node that provides additional information using [[valueReference]].\n* Note 3: Note that you can use [[minValue]] and [[maxValue]] to indicate ranges.
struct weightTotal;

/// A link to a screenshot image of the app.
struct screenshot;

/// Device required to run the application. Used in cases where a specific make/model is required to run the application.
struct availableOnDevice;

/// The 14-character, HIOS-generated Plan ID number. (Plan IDs must be unique, even across different markets.)
struct healthPlanId;

/// The location in which the strength is available.
struct availableIn;

/// What type of code sample: full (compile ready) solution, code snippet, inline code, scripts, template.
struct sampleType;

/// The male gender.
struct Male;

/// The International Standard Musical Work Code for the composition.
struct iswcCode;

/// Nonprofit501c9: Non-profit type referring to Voluntary Employee Beneficiary Associations.
struct Nonprofit501c9;

/// The date and time the reservation was modified.
struct modifiedTime;

/// A [hackathon](https://en.wikipedia.org/wiki/Hackathon) event.
struct Hackathon;

/// The act of gaining ownership of an object from an origin. Reciprocal of GiveAction.\n\nRelated actions:\n\n* [[GiveAction]]: The reciprocal of TakeAction.\n* [[ReceiveAction]]: Unlike ReceiveAction, TakeAction implies that ownership has been transfered.
struct TakeAction;

/// The current approximate inventory level for the item or items.
struct inventoryLevel;

/// An historical landmark or building.
struct LandmarksOrHistoricalBuildings;

/// An international trial.
struct InternationalTrial;

/// Data type: Floating number.
struct Float;

/// The specific biochemical interaction through which this drug or supplement produces its pharmacological effect.
struct mechanismOfAction;

/// Date on which this guideline's recommendation was made.
struct guidelineDate;

/// Indicates the department, unit and/or facility where the employee reports and/or in which the job is to be performed.
struct employmentUnit;

/// The number of axles.\n\nTypical unit code(s): C62
struct numberOfAxles;

/// OrderStatus representing that an order is in transit.
struct OrderInTransit;

/// The opening hour of the place or service on the given day(s) of the week.
struct opens;

/// A NewsArticle associated with the Media Object.
struct associatedArticle;

/// A URL to a map of the place.
struct map;

/// A video file.
struct VideoObject;

/// Description of benefits associated with the job.
struct benefits;

/// Identifier of the flight's departure terminal.
struct departureTerminal;

/// Data derived from multiple randomized clinical trials or meta-analyses.
struct EvidenceLevelA;

/// Any precaution, guidance, contraindication, etc. related to consumption of specific foods while taking this drug.
struct foodWarning;

/// Indicates the property or properties by which the variants in a [[ProductGroup]] vary, e.g. their size, color etc. Schema.org properties can be referenced by their short name e.g. "color"; terms defined elsewhere can be referenced with their URIs.
struct variesBy;

/// A sub property of participant. The participant/person/organization that bought the object.
struct buyer;

/// The party holding the legal copyright to the CreativeWork.
struct copyrightHolder;

/// CreativeWorkSeries dedicated to TV broadcast and associated online delivery.
struct TVSeries;

/// The date and place the work was first performed.
struct firstPerformance;

/// The amount of time in a term as defined by the institution. A term is a length of time where students take one or more classes. Semesters and quarters are common units for term.
struct termDuration;

/// The longitude of a location. For example ```-122.08585``` ([WGS 84](https://en.wikipedia.org/wiki/World_Geodetic_System)).
struct longitude;

/// Supporting data for a SoftwareApplication.
struct supportingData;

/// A hotel room is a single room in a hotel.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct HotelRoom;

/// The number of times terms of study are offered per year. Semesters and quarters are common units for term. For example, if the student can only take 2 semesters for the program in one year, then termsPerYear should be 2.
struct termsPerYear;

/// An item within in a data feed. Data feeds may have many elements.
struct dataFeedElement;

/// Indicates whether it is allowed to smoke in the place, e.g. in the restaurant, hotel or hotel room.
struct smokingAllowed;

/// The act of transferring ownership of an object to a destination. Reciprocal of TakeAction.\n\nRelated actions:\n\n* [[TakeAction]]: Reciprocal of GiveAction.\n* [[SendAction]]: Unlike SendAction, GiveAction implies that ownership is being transferred (e.g. I may send my laptop to you, but that doesn't mean I'm giving it to you).
struct GiveAction;

/// A performance group, such as a band, an orchestra, or a circus.
struct PerformingGroup;

/// A sub-grouping of food or drink items in a menu. E.g. courses (such as 'Dinner', 'Breakfast', etc.), specific type of dishes (such as 'Meat', 'Vegan', 'Drinks', etc.), or some other classification made by the menu provider.
struct MenuSection;

/// The position of the steering wheel or similar device (mostly for cars).
struct steeringPosition;

/// One of the more significant URLs on the page. Typically, these are the non-navigation links that are clicked on the most.
struct significantLink;

/// A pointer to products or services sought by the organization or person (demand).
struct seeks;

/// Indicate how many people can play this game (minimum, maximum, or range).
struct numberOfPlayers;

/// The electronic systems used to play <a href="http://en.wikipedia.org/wiki/Category:Video_game_platforms">video games</a>.
struct gamePlatform;

/// A URL pointing to a player for a specific video. In general, this is the information in the ```src``` element of an ```embed``` tag and should not be the same as the content of the ```loc``` tag.
struct embedUrl;

/// The number of membership points earned by the member. If necessary, the unitText can be used to express the units the points are issued in. (e.g. stars, miles, etc.)
struct membershipPointsEarned;

/// The number of rooms (excluding bathrooms and closets) of the accommodation or lodging business.
/// Typical unit code(s): ROM for room or C62 for no unit. The type of room can be put in the unitText property of the QuantitativeValue.
struct numberOfRooms;

/// A day spa.
struct DaySpa;

/// A geographical region, typically under the jurisdiction of a particular government.
struct AdministrativeArea;

/// The permitted weight of a trailer attached to the vehicle.\n\nTypical unit code(s): KGM for kilogram, LBR for pound\n* Note 1: You can indicate additional information in the [[name]] of the [[QuantitativeValue]] node.\n* Note 2: You may also link to a [[QualitativeValue]] node that provides additional information using [[valueReference]].\n* Note 3: Note that you can use [[minValue]] and [[maxValue]] to indicate ranges.
struct trailerWeight;

/// Any part of the human body, typically a component of an anatomical system. Organs, tissues, and cells are all anatomical structures.
struct AnatomicalStructure;

/// A reservoir of water, typically an artificially created lake, like the Lake Kariba reservoir.
struct Reservoir;

/// An explanation in the instructions for how to achieve a result. It provides supplementary information about a technique, supply, author's preference, etc. It can explain what could be done, or what should not be done, but doesn't specify what should be done (see HowToDirection).
struct HowToTip;

/// Organizations that the person works for.
struct worksFor;

/// The floor level for an [[Accommodation]] in a multi-storey building. Since counting
///   systems [vary internationally](https://en.wikipedia.org/wiki/Storey#Consecutive_number_floor_designations), the local system should be used where possible.
struct floorLevel;

/// A shop that sells alcoholic drinks such as wine, beer, whisky and other spirits.
struct LiquorStore;

/// The act of producing a balanced opinion about the object for an audience. An agent reviews an object with participants resulting in a review.
struct ReviewAction;

/// An order is a confirmation of a transaction (a receipt), which can contain multiple line items, each represented by an Offer that has been accepted by the customer.
struct Order;

/// After this date, the item will no longer be available for pickup.
struct availableThrough;

/// A movie rental store.
struct MovieRentalStore;

/// The highest price if the price is a range.
struct maxPrice;

/// A picture or diagram made with a pencil, pen, or crayon rather than paint.
struct Drawing;

/// A tourist destination. In principle any [[Place]] can be a [[TouristDestination]] from a [[City]], [[Region]] or [[Country]] to an [[AmusementPark]] or [[Hotel]]. This Type can be used on its own to describe a general [[TouristDestination]], or be used as an [[additionalType]] to add tourist relevant properties to any other [[Place]].  A [[TouristDestination]] is defined as a [[Place]] that contains, or is colocated with, one or more [[TouristAttraction]]s, often linked by a similar theme or interest to a particular [[touristType]]. The [UNWTO](http://www2.unwto.org/) defines Destination (main destination of a tourism trip) as the place visited that is central to the decision to take the trip.
///   (See examples below).
struct TouristDestination;

/// Musculoskeletal system clinical examination.
struct MusculoskeletalExam;

/// A cardholder benefit that pays the cardholder a small percentage of their net expenditures.
struct cashBack;

/// Indicates data describing a hospital, e.g. a CDC [[CDCPMDRecord]] or as some kind of [[Dataset]].
struct healthcareReportingData;

/// The merchantReturnDays property indicates the number of days (from purchase) within which relevant merchant return policy is applicable.
struct merchantReturnDays;

/// The name of the bus (e.g. Bolt Express).
struct busName;

/// For an [[Organization]] (often but not necessarily a [[NewsMediaOrganization]]), a description of organizational ownership structure; funding and grants. In a news/media setting, this is with particular reference to editorial independence.   Note that the [[funder]] is also available and can be used to make basic funder information machine-readable.
struct ownershipFundingInfo;

/// The act of consuming static visual content.
struct ViewAction;

/// Indicates the number of constraints (not counting [[populationType]]) defined for a particular [[StatisticalPopulation]]. This helps applications understand if they have access to a sufficiently complete description of a [[StatisticalPopulation]].
struct numConstraints;

/// Represents spatial relations in which two geometries (or the places they represent) have at least one point in common. As defined in [DE-9IM](https://en.wikipedia.org/wiki/DE-9IM).
struct geoIntersects;

/// An ocean (for example, the Pacific).
struct OceanBodyOfWater;

/// An automatic payment system is in place and will be used.
struct PaymentAutomaticallyApplied;

/// If applicable, a medical specialty in which this entity is relevant.
struct relevantSpecialty;

/// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
struct interactionStatistic;

/// Statistical information about the spread of a disease, either as [[WebContent]], or
///   described directly as a [[Dataset]], or the specific [[Observation]]s in the dataset. When a [[WebContent]] URL is
///   provided, the page indicated might also contain more such markup.
struct diseaseSpreadStatistics;

/// URL at which the app may be installed, if different from the URL of the item.
struct installUrl;

/// The conventional Western system of medicine, that aims to apply the best available evidence gained from the scientific method to clinical decision making. Also known as conventional or Western medicine.
struct WesternConventional;

/// A medical condition associated with this anatomy.
struct relatedCondition;

/// Medical expert advice related to the plan.
struct expertConsiderations;

/// A trial that takes place at multiple centers.
struct MultiCenterTrial;

/// A preventative therapy used to prevent an initial occurrence of the medical condition, such as vaccination.
struct primaryPrevention;

/// The Manufacturer Part Number (MPN) of the product, or the product to which the offer refers.
struct mpn;

/// Nonprofit501c1: Non-profit type referring to Corporations Organized Under Act of Congress, including Federal Credit Unions and National Farm Loan Associations.
struct Nonprofit501c1;

/// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
struct code;

/// Any membership in a frequent flyer, hotel loyalty program, etc. being applied to the reservation.
struct programMembershipUsed;

/// Residence type: Apartment complex.
struct ApartmentComplex;

/// A DeliveryMethod in which an item is made available via locker.
struct LockerDelivery;

/// Nonprofit501n: Non-profit type referring to Charitable Risk Pools.
struct Nonprofit501n;

/// Indicates whether a FoodEstablishment accepts reservations. Values can be Boolean, an URL at which reservations can be made or (for backwards compatibility) the strings ```Yes``` or ```No```.
struct acceptsReservations;

/// The drug's cost represents the maximum reimbursement paid by an insurer for the drug.
struct ReimbursementCap;

/// A camping site, campsite, or [[Campground]] is a place used for overnight stay in the outdoors, typically containing individual [[CampingPitch]] locations. \n\n
/// In British English a campsite is an area, usually divided into a number of pitches, where people can camp overnight using tents or camper vans or caravans; this British English use of the word is synonymous with the American English expression campground. In American English the term campsite generally means an area where an individual, family, group, or military unit can pitch a tent or park a camper; a campground may contain many campsites (Source: Wikipedia see [https://en.wikipedia.org/wiki/Campsite](https://en.wikipedia.org/wiki/Campsite)).\n\n
/// 
/// See also the dedicated [document on the use of schema.org for marking up hotels and other forms of accommodations](/docs/hotels.html).
struct Campground;

/// A product or service offered by a bank whereby one may deposit, withdraw or transfer money and in some cases be paid interest.
struct BankAccount;

/// The unique identifier for a flight including the airline IATA code. For example, if describing United flight 110, where the IATA code for United is 'UA', the flightNumber is 'UA110'.
struct flightNumber;

/// A facility, often associated with a hospital or medical school, that is devoted to the specific diagnosis and/or healthcare. Previously limited to outpatients but with evolution it may be open to inpatients as well.
struct MedicalClinic;

/// A radio channel that uses FM.
struct FMRadioChannel;

/// Permission to add comments to the document.
struct CommentPermission;

/// Whether The costs to the patient for services under this network or formulary.
struct healthPlanCostSharing;

/// Another drug that is known to interact with this drug in a way that impacts the effect of this drug or causes a risk to the patient. Note: disease interactions are typically captured as contraindications.
struct interactingDrug;

/// A combination of date and time of day in the form [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm] (see Chapter 5.4 of ISO 8601).
struct DateTime;

/// A material that something is made from, e.g. leather, wool, cotton, paper.
struct material;

/// The product identifier, such as ISBN. For example: ``` meta itemprop="productID" content="isbn:123-456-789" ```.
struct productID;

/// MixedEventAttendanceMode - an event that is conducted as a combination of both offline and online modes.
struct MixedEventAttendanceMode;

/// A DatedMoneySpecification represents monetary values with optional start and end dates. For example, this could represent an employee's salary over a specific period of time. __Note:__ This type has been superseded by [[MonetaryAmount]] use of that type is recommended
struct DatedMoneySpecification;

/// A food or drink item listed in a menu or menu section.
struct MenuItem;

/// Indicates that the item is refurbished.
struct RefurbishedCondition;

/// An identifier for the legislation. This can be either a string-based identifier, like the CELEX at EU level or the NOR in France, or a web-based, URL/URI identifier, like an ELI (European Legislation Identifier) or an URN-Lex.
struct legislationIdentifier;

/// A reservation for train travel.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
struct TrainReservation;

/// An indication for a medical therapy that has been formally specified or approved by a regulatory body that regulates use of the therapy; for example, the US FDA approves indications for most drugs in the US.
struct ApprovedIndication;

/// A page devoted to a single item, such as a particular product or hotel.
struct ItemPage;

/// A college, university, or other third-level educational institution.
struct CollegeOrUniversity;

/// The catalog number for the release.
struct catalogNumber;

/// The insurance plans that cover this drug.
struct includedInHealthInsurancePlan;

/// A property-value pair, e.g. representing a feature of a product or place. Use the 'name' property for the name of the property. If there is an additional human-readable version of the value, put that into the 'description' property.\n\n Always use specific schema.org properties when a) they exist and b) you can populate them. Using PropertyValue as a substitute will typically not trigger the same effect as using the original, specific property.
///     
struct PropertyValue;

/// NLNonprofitType: Non-profit organization type originating from the Netherlands.
struct NLNonprofitType;

/// The specific time described by a creative work, for works (e.g. articles, video objects etc.) that emphasise a particular moment within an Event.
struct contentReferenceTime;

/// The difference between the price at which a broker or other intermediary buys and sells foreign currency.
struct exchangeRateSpread;

/// A CreativeWorkSeries in schema.org is a group of related items, typically but not necessarily of the same kind. CreativeWorkSeries are usually organized into some order, often chronological. Unlike [[ItemList]] which is a general purpose data structure for lists of things, the emphasis with CreativeWorkSeries is on published materials (written e.g. books and periodicals, or media such as tv, radio and games).\n\nSpecific subtypes are available for describing [[TVSeries]], [[RadioSeries]], [[MovieSeries]], [[BookSeries]], [[Periodical]] and [[VideoGameSeries]]. In each case, the [[hasPart]] / [[isPartOf]] properties can be used to relate the CreativeWorkSeries to its parts. The general CreativeWorkSeries type serves largely just to organize these more specific and practical subtypes.\n\nIt is common for properties applicable to an item from the series to be usefully applied to the containing group. Schema.org attempts to anticipate some of these cases, but publishers should be free to apply properties of the series parts to the series as a whole wherever they seem appropriate.
/// 	  
struct CreativeWorkSeries;

/// An aspect of medical practice that is considered on the page, such as 'diagnosis', 'treatment', 'causes', 'prognosis', 'etiology', 'epidemiology', etc.
struct aspect;

/// The therapy that is concerned with the maintenance or improvement of respiratory function (as in patients with pulmonary disease).
struct RespiratoryTherapy;

/// BroadcastRelease.
struct BroadcastRelease;

/// The larger organization that this local business is a branch of, if any. Not to be confused with (anatomical)[[branch]].
struct branchOf;

/// A sub property of instrument. The diet used in this action.
struct diet;

/// The TV series to which this episode or season belongs.
struct partOfTVSeries;

/// A restaurant.
struct Restaurant;

/// A specific branch of medical science that pertains to diagnosis and treatment of disorders of endocrine glands and their secretions.
struct Endocrine;

/// A motel.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
struct Motel;

/// A refundType, from an enumerated list.
struct refundType;

/// The permitted weight of passengers and cargo, EXCLUDING the weight of the empty vehicle.\n\nTypical unit code(s): KGM for kilogram, LBR for pound\n\n* Note 1: Many databases specify the permitted TOTAL weight instead, which is the sum of [[weight]] and [[payload]]\n* Note 2: You can indicate additional information in the [[name]] of the [[QuantitativeValue]] node.\n* Note 3: You may also link to a [[QualitativeValue]] node that provides additional information using [[valueReference]].\n* Note 4: Note that you can use [[minValue]] and [[maxValue]] to indicate ranges.
struct payload;

/// A music venue.
struct MusicVenue;

/// Book format: Ebook.
struct EBook;

/// The act of intentionally disregarding the object. An agent ignores an object.
struct IgnoreAction;

/// Indicates whether some facility (e.g. [[FoodEstablishment]], [[CovidTestingFacility]]) offers a service that can be used by driving through in a car. In the case of [[CovidTestingFacility]] such facilities could potentially help with social distancing from other potentially-infected users.
struct hasDriveThroughService;

/// Defines the energy efficiency Category (which could be either a rating out of range of values or a yes/no certification) for a product according to an international energy efficiency standard
struct hasEnergyEfficiencyCategory;

/// An car dealership.
struct AutoDealer;

/// Categories of physical activity, organized by physiologic classification.
struct PhysicalActivityCategory;

/// A returnPolicyCategory expresses at most one of several enumerated kinds of return.
struct returnPolicyCategory;

/// A license document that applies to this structured data, typically indicated by URL.
struct sdLicense;

/// An individual or organization that has some kind of responsibility for the legislation. Typically the ministry who is/was in charge of elaborating the legislation, or the adressee for potential questions about the legislation once it is published.
struct legislationResponsible;

/// Nonprofit501c23: Non-profit type referring to Veterans Organizations.
struct Nonprofit501c23;

/// A DeliveryMethod in which an item is collected on site, e.g. in a store or at a box office.
struct OnSitePickup;

/// A doctor's office.
struct Physician;

/// Nonprofit501c22: Non-profit type referring to Withdrawal Liability Payment Funds.
struct Nonprofit501c22;

/// The act of asserting that a future event/action is no longer going to happen.\n\nRelated actions:\n\n* [[ConfirmAction]]: The antonym of CancelAction.
struct CancelAction;

/// A FundingScheme combines organizational, project and policy aspects of grant-based funding
///     that sets guidelines, principles and mechanisms to support other kinds of projects and activities.
///     Funding is typically organized via [[Grant]] funding. Examples of funding schemes: Swiss Priority Programmes (SPPs); EU Framework 7 (FP7); Horizon 2020; the NIH-R01 Grant Program; Wellcome institutional strategic support fund. For large scale public sector funding, the management and administration of grant awards is often handled by other, dedicated, organizations - [[FundingAgency]]s such as ERC, REA, ...
struct FundingScheme;

/// A structured value indicating the quantity, unit of measurement, and business function of goods included in a bundle offer.
struct TypeAndQuantityNode;

/// A [[LibrarySystem]] is a collaborative system amongst several libraries.
struct LibrarySystem;

/// A hospital.
struct Hospital;

/// A language someone may use with or at the item, service or place. Please use one of the language codes from the [IETF BCP 47 standard](http://tools.ietf.org/html/bcp47). See also [[inLanguage]]
struct availableLanguage;

/// Ear function assessment with clinical examination.
struct Ear;

/// The maximum dosing schedule considered safe for a drug or supplement as recommended by an authority or by the drug/supplement's manufacturer. Capture the recommending authority in the recognizingAuthority property of MedicalEntity.
struct MaximumDoseSchedule;

/// The person's spouse.
struct spouse;

/// The count of total number of ratings.
struct ratingCount;

/// Organization or person who adapts a creative work to different languages, regional differences and technical requirements of a target market, or that translates during some event.
struct translator;

/// The type of a bank account.
struct bankAccountType;

/// A demand entity represents the public, not necessarily binding, not necessarily exclusive, announcement by an organization or person to seek a certain type of goods or services. For describing demand using this type, the very same properties used for Offer apply.
struct Demand;

/// A chemical or biologic substance, used as a medical therapy, that has a physiological effect on an organism. Here the term drug is used interchangeably with the term medicine although clinical knowledge make a clear difference between them.
struct Drug;

/// The act of starting or activating a device or application (e.g. starting a timer or turning on a flashlight).
struct ActivateAction;

/// The condition, complication, symptom, sign, etc. caused.
struct causeOf;