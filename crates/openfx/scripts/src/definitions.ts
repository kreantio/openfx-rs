export interface CodegenConfig {
  property_names: {
    simple_names_regex: string;
    simple_names_special_cases: Record<string, string>;
  };
  actions: {
    pseudo: {
      members: string[];
    };
    core: {
      members_regex: string;
    };
    image_effect: {
      members_regex: string;
      members_from_core: string[];
    };
    interact: {
      members_regex: string;
      members_from_core: string[];
    };
  };
}
