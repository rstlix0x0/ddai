use tracing::{info, instrument};

use crate::core::types::validate;

use super::types::{Builder, Desc, Name, Project, ProjectError};

#[derive(Debug, Clone)]
pub(crate) struct App<T>
where
    T: Builder,
{
    builder: T,
}

impl<T> App<T>
where
    T: Builder,
{
    pub fn new(builder: T) -> Self {
        App { builder }
    }

    #[instrument(skip_all, err)]
    pub fn init(&self, name: Name, desc: Option<Desc>) -> Result<(), ProjectError> {
        info!("Initializing project with name: {}", name.as_str());
        let project = Project::new(name, desc);

        info!("Validating project");
        let _ = validate(&project).map_err(|err| ProjectError::ValidationError(err))?;

        info!("Project validation successful, start build project");
        self.builder
            .initiate(project)
            .map_err(|err| ProjectError::InitiateError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock!(
        FakeAppBuilder{}

        impl Builder for FakeAppBuilder {
            fn initiate(&self, project: Project) -> Result<(), ProjectError>;
        }
    );

    mod expect_errors {
        use super::*;

        #[test]
        fn test_fail_on_validation() {
            let mut builder = MockFakeAppBuilder::new();
            builder.expect_initiate().returning(|_| Ok(()));

            let app = App::new(builder);
            let name = Name::from(""); // Empty name to trigger validation error
            let desc = Some(Desc::from("This is a test project"));

            let result = app.init(name, desc);
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert!(matches!(err, ProjectError::ValidationError(_)));
            match err {
                ProjectError::ValidationError(msg) => {
                    assert!(msg.to_string().contains("Project name cannot be empty"))
                }
                _ => panic!("Expected ValidationError"),
            }
        }

        #[test]
        fn test_initiate_error() {
            let mut builder = MockFakeAppBuilder::new();
            builder.expect_initiate().returning(|_| {
                Err(ProjectError::InitiateError(
                    "Failed to initiate".to_string(),
                ))
            });

            let app = App::new(builder);
            let name = Name::from("Test Project");
            let desc = Some(Desc::from("This is a test project"));

            let result = app.init(name, desc);
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert!(matches!(err, ProjectError::InitiateError(_)));
            match err {
                ProjectError::InitiateError(msg) => assert!(msg.contains("Failed to initiate")),
                _ => panic!("Expected InitiateError"),
            }
        }
    }

    #[test]
    fn test_successful_initiation() {
        let mut builder = MockFakeAppBuilder::new();
        builder.expect_initiate().returning(|_| Ok(()));

        let app = App::new(builder);
        let name = Name::from("Test Project");
        let desc = Some(Desc::from("This is a test project"));

        let result = app.init(name, desc);
        assert!(result.is_ok());
    }
}
